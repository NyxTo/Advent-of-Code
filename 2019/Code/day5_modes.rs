use std::fs::read_to_string;

fn param(prog: &Vec<i32>, pos: usize, i: usize) -> i32 {
	match prog[pos] / 10_i32.pow(i as u32 + 1) % 10 {
		0 => prog[prog[pos + i] as usize],
		1 => prog[pos + i],
		_ => panic!("invalid parameter mode"),
	}
}

fn run(mut prog: Vec<i32>, id: i32) -> i32 {
	let (mut pos, mut has_in, mut has_out, mut diag) = (0, false, false, 0);
	loop {
		let op = prog[pos];
		if op == 99 { break; }
		match op % 100 {
			1 => {
				let store = prog[pos + 3] as usize;
				prog[store] = param(&prog, pos, 1) + param(&prog, pos, 2);
				pos += 4;
			},
			2 => {
				let store = prog[pos + 3] as usize;
				prog[store] = param(&prog, pos, 1) * param(&prog, pos, 2);
				pos += 4;
			},
			3 => {
				let store = prog[pos + 1] as usize;
				if !has_in {
					prog[store] = id;
					has_in = true;
				}
				else { panic!("too many input instructions"); }
				pos += 2;
			},
			4 => {
				let output = param(&prog, pos, 1);
				if has_out { panic!("diagnostic code not final output"); }
				else if output != 0 {
					diag = output;
					has_out = true;
				}
				pos += 2;
			},
			5 => pos = if param(&prog, pos, 1) != 0 { param(&prog, pos, 2) as usize } else { pos + 3 },
			6 => pos = if param(&prog, pos, 1) == 0 { param(&prog, pos, 2) as usize } else { pos + 3 },
			7 => {
				let store = prog[pos + 3] as usize;
				prog[store] = if param(&prog, pos, 1) < param(&prog, pos, 2) { 1 } else { 0 };
				pos += 4;
			},
			8 => {
				let store = prog[pos + 3] as usize;
				prog[store] = if param(&prog, pos, 1) == param(&prog, pos, 2) { 1 } else { 0 };
				pos += 4;
			},
			_ => panic!("invalid opcode"),
		}
	}
	if !has_out { panic!("no diagnostic code"); }
	diag
}

fn main() {
	let prog = read_to_string("in5.txt").unwrap().trim_end().split(',').map(|int| int.parse::<i32>().unwrap()).collect::<Vec<_>>();
	println!("Part A: {}", run(prog.clone(), 1)); // 15259545
	println!("Part B: {}", run(prog.clone(), 5)); // 7616021
}
