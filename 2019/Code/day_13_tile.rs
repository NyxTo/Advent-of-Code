use std::fs::read_to_string;

fn read_param(prog: &Vec<i32>, base: i32, pos: usize, i: usize) -> i32 {
	match prog[pos] / 10_i32.pow(i as u32 + 1) % 10 {
		0 => prog[prog[pos + i] as usize],
		1 => prog[pos + i],
		2 => prog[(base + prog[pos + i]) as usize],
		_ => panic!("invalid parameter mode"),
	}
}

fn write_param(prog: &Vec<i32>, base: i32, pos: usize, i: usize) -> usize {
	match prog[pos] / 10_i32.pow(i as u32 + 1) % 10 {
		0 => prog[pos + i] as usize,
		2 => (base + prog[pos + i]) as usize,
		_ => panic!("invalid parameter mode"),
	}
}

fn run(mut prog: Vec<i32>, quart: bool) -> (i32, i32) {
	let (mut pos, mut base, mut grid, mut coords, mut wait, mut blocks, mut padl, mut ball, mut score) = (0, 0, [[5; 42]; 24], (0, 0), 0, 0, 0, 0, 0);
	if quart { prog[0] = 2; }
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
				if !quart { panic!("no input given"); }
				let store = write_param(&prog, base, pos, 1);
				prog[store] = if ball < padl { -1 } else if ball > padl { 1 } else { 0 };
				pos += 2;
			},
			4 => {
				let output = read_param(&prog, base, pos, 1);
				if wait == 0 { coords.0 = output; }
				else if wait == 1 { coords.1 = output; }
				else if wait == 2 {
					if coords.0 == -1 && coords.1 == 0 { score = output; }
					else {
						grid[coords.1 as usize][coords.0 as usize] = output;
						if !quart && output == 2 { blocks += 1; }
						if output == 3 { padl = coords.0 };
						if output == 4 { ball = coords.0 };
					}
				}
				wait = (wait + 1) % 3;
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
	(blocks, score)
}

fn main() {
	let mut prog = read_to_string("in_13.txt").unwrap().trim_end().split(',').map(|int| int.parse::<i32>().unwrap()).collect::<Vec<_>>();
	prog.resize(2700, 0);
	println!("Part A: {}", run(prog.clone(), false).0); // 258
	println!("Part B: {}", run(prog.clone(), true).1); // 12765
}

