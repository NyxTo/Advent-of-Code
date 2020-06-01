use std::fs::read_to_string;
use std::cmp::min;
use std::collections::VecDeque;

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

fn run(mut prog: Vec<i32>) -> ((usize, usize), [[i32; 43]; 43]) {
	let (mut pos, mut base, mut dist, mut trail, mut coords, mut oxygen) = (0, 0, [[1400; 43]; 43], vec![], (21, 21), (0, 0));
	dist[coords.1][coords.0] = 0;
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
				if coords.1 > 0 && dist[coords.1 - 1][coords.0] == 1400 {
					prog[store] = 1;
					trail.push((coords.0, coords.1, 2));
					coords.1 -= 1;
				} else if coords.1 < 42 && dist[coords.1 + 1][coords.0] == 1400 {
					prog[store] = 2;
					trail.push((coords.0, coords.1, 1));
					coords.1 += 1;
				} else if coords.0 > 0 && dist[coords.1][coords.0 - 1] == 1400 {
					prog[store] = 3;
					trail.push((coords.0, coords.1, 4));
					coords.0 -= 1;
				} else if coords.0 < 42 && dist[coords.1][coords.0 + 1] == 1400 {
					prog[store] = 4;
					trail.push((coords.0, coords.1, 3));
					coords.0 += 1;
				} else {
					let prev = trail.pop().unwrap();
					coords = (prev.0, prev.1);
					prog[store] = prev.2;
				}
				pos += 2;
			},
			4 => {
				match read_param(&prog, base, pos, 1) {
					0 => {
						dist[coords.1][coords.0] = -1;
						let prev = trail.pop().unwrap();
						coords = (prev.0, prev.1);
					},
					1 => {
						if trail.is_empty() { break; }
						let prev = trail[trail.len() - 1];
						dist[coords.1][coords.0] = min(dist[coords.1][coords.0], dist[prev.1][prev.0] + 1);
					},
					2 => {
						let prev = trail[trail.len() - 1];
						dist[coords.1][coords.0] = min(dist[coords.1][coords.0], dist[prev.1][prev.0] + 1);
						oxygen = coords;
					},
					_ => panic!("invalid status"),
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
	(oxygen, dist)
}

fn main() {
	let mut prog = read_to_string("in_15.txt").unwrap().trim_end().split(',').map(|int| int.parse::<i32>().unwrap()).collect::<Vec<_>>();
	prog.resize(2700, 0);
	let ((oxygen, dist), mut bfs, mut visit, mut time) = (run(prog), VecDeque::new(), [[false; 43]; 43], 0);
	println!("Part A: {}", dist[oxygen.1][oxygen.0]); // 230
	bfs.push_back((oxygen.0, oxygen.1, 0));
	while !bfs.is_empty() {
		let node = bfs.pop_front().unwrap();
		visit[node.1][node.0] = true;
		time = node.2;
		if node.1 > 0 && dist[node.1 - 1][node.0] >= 0 && !visit[node.1 - 1][node.0] { bfs.push_back((node.0, node.1 - 1, node.2 + 1)); }
		if node.1 < 42 && dist[node.1 + 1][node.0] >= 0 && !visit[node.1 + 1][node.0] { bfs.push_back((node.0, node.1 + 1, node.2 + 1)); }
		if node.0 > 0 && dist[node.1][node.0 - 1] >= 0 && !visit[node.1][node.0 - 1] { bfs.push_back((node.0 - 1, node.1, node.2 + 1)); }
		if node.0 < 42 && dist[node.1][node.0 + 1] >= 0 && !visit[node.1][node.0 + 1] { bfs.push_back((node.0 + 1, node.1, node.2 + 1)); }
	}
	println!("Part B: {}", time); // 288
}
