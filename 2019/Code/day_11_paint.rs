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
	match prog[pos] / 10_i64.pow(i as u32 + 1) % 10 {
		0 => prog[pos + i] as usize,
		2 => (base + prog[pos + i]) as usize,
		_ => panic!("invalid parameter mode"),
	}
}

fn run(mut prog: Vec<i64>, panel: bool) -> ([[i64; 90]; 60], [[bool; 90]; 60]) {
	let (mut pos, mut base, mut grid, mut paint, mut coords, mut dir, mut wait) = (0, 0, [[0; 90]; 60], [[false; 90]; 60], (40, 20), 0, true);
	grid[coords.1][coords.0] = if panel { 1 } else { 0 };
	paint[coords.1][coords.0] = panel;
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
				prog[store] = grid[coords.1][coords.0];
				pos += 2;
			},
			4 => {
				let output = read_param(&prog, base, pos, 1);
				if wait {
					grid[coords.1][coords.0] = output;
					paint[coords.1][coords.0] = true;
				} else {
					dir = match output {
						0 => (dir + 1) % 4,
						1 => (dir + 3) % 4,
						_ => panic!("invalid output"),
					};
					match dir {
						0 => coords.1 -= 1,
						1 => coords.0 -= 1,
						2 => coords.1 += 1,
						3 => coords.0 += 1,
						_ => panic!("invalid direction"),
					};
				}
				wait = !wait;
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
	(grid, paint)
}

fn main() {
	let mut prog = read_to_string("in_11.txt").unwrap().trim_end().split(',').map(|int| int.parse::<i64>().unwrap()).collect::<Vec<_>>();
	prog.resize(1200, 0);
	let (paint, mut count) = (run(prog.clone(), false).1, 0);
	for y in 0..60 {
	for x in 0..90 {
		if paint[y][x] { count += 1; }
	}
	}
	println!("Part A: {}", count); // 1885
	let grid = run(prog.clone(), true).0;
	println!("Part B:"); // BFEAGHAF
	for y in 0..60 {							//  # # #     # # # #   # # # #     # #       # #     #     #     # #     # # # #
		for x in 0..90 {						//  #     #   #         #         #     #   #     #   #     #   #     #   #      
			print!(" {}", if grid[y][x] == 1 { '#' } else { ' ' });	//  # # #     # # #     # # #     #     #   #         # # # #   #     #   # # #  
		}								//  #     #   #         #         # # # #   #   # #   #     #   # # # #   #      
		println!();							//  #     #   #         #         #     #   #     #   #     #   #     #   #      
	}									//  # # #     #         # # # #   #     #     # # #   #     #   #     #   #      
}
