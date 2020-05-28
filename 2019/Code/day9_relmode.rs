use std::fs::read_to_string;

fn read_param(prog: &Vec<i64>, base: i64, pos: usize, i: usize) -> i64 {
	match prog[pos] / 10_i64.pow(i as u32 + 1) % 10 {
		0 => prog[prog[pos + i] as usize],
		1 => prog[pos + i],
		2 => prog[(base + prog[pos + i]) as usize],
		_ => panic!("invalid parameter mode"),
	}
}

fn write_param(prog: &Vec<i64>, base: i64, pos: usize, i: usize) -> usize {
	(match prog[pos] / 10_i64.pow(i as u32 + 1) % 10 {
		0 => prog[pos + i],
		2 => base + prog[pos + i],
		_ => panic!("invalid parameter mode"),
	}) as usize
}

fn run(mut prog: Vec<i64>, single_in: i64) -> i64 {
	let (mut pos, mut base, mut has_in, mut has_out, mut single_out) = (0, 0, false, false, 0);
	loop {
		let op = prog[pos];
		if op == 99 { break; }
		match op % 100 {
			1 => {
				let store = write_param(&prog, base, pos, 3);
				prog[store] = read_param(&prog, base, pos, 1) + read_param(&prog, base, pos, 2);
				pos += 4;
			},
			2 => {
				let store = write_param(&prog, base, pos, 3);
				prog[store] = read_param(&prog, base, pos, 1) * read_param(&prog, base, pos, 2);
				pos += 4;
			},
			3 => {
				let store = write_param(&prog, base, pos, 1);
				if !has_in {
					prog[store] = single_in;
					has_in = true;
				}
				else { panic!("too many input instructions"); }
				pos += 2;
			},
			4 => {
				let output = read_param(&prog, base, pos, 1);
				if has_out { panic!("boost keycode not final output"); }
				else {
					single_out = output;
					has_out = true;
				}
				pos += 2;
			},
			5 => pos = if read_param(&prog, base, pos, 1) != 0 { read_param(&prog, base, pos, 2) as usize } else { pos + 3 },
			6 => pos = if read_param(&prog, base, pos, 1) == 0 { read_param(&prog, base, pos, 2) as usize } else { pos + 3 },
			7 => {
				let store = write_param(&prog, base, pos, 3);
				prog[store] = if read_param(&prog, base, pos, 1) < read_param(&prog, base, pos, 2) { 1 } else { 0 };
				pos += 4;
			},
			8 => {
				let store = write_param(&prog, base, pos, 3);
				prog[store] = if read_param(&prog, base, pos, 1) == read_param(&prog, base, pos, 2) { 1 } else { 0 };
				pos += 4;
			},
			9 => {
				base += read_param(&prog, base, pos, 1);
				pos += 2;
			}
			_ => panic!("invalid opcode"),
		}
	}
	if !has_out { panic!("no output"); }
	single_out
}

fn main() {
	let mut prog = read_to_string("in9.txt").unwrap().trim_end().split(',').map(|int| int.parse::<i64>().unwrap()).collect::<Vec<_>>();
	prog.resize(1100, 0);
	println!("Part A: {}", run(prog.clone(), 1)); // 2662308295
	println!("Part B: {}", run(prog.clone(), 2)); // 63441
}
