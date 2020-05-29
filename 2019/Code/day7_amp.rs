use std::fs::read_to_string;
use std::cmp::max;

fn param(prog: &Vec<i32>, pos: usize, i: usize) -> i32 {
	match prog[pos] / 10_i32.pow(i as u32 + 1) % 10 {
		0 => prog[prog[pos + i] as usize],
		1 => prog[pos + i],
		_ => panic!("invalid parameter mode"),
	}
}

fn run(prog: &mut Vec<i32>, mut pos: usize, mut has_in: bool, phase: i32, sig: i32) -> (usize, bool, Option<i32>) {
	loop {
		let op = prog[pos];
		if op == 99 { return (pos, has_in, None); }
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
				prog[store] = if !has_in { phase } else { sig };
				has_in = true;
				pos += 2;
			},
			4 => {
				let output = param(&prog, pos, 1);
				pos += 2;
				return (pos, has_in, Some(output));
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
}

fn next_perm(perm: &mut [i32; 5]) -> bool {
	let (mut pivot, mut succ) = (4, 4);
	while pivot > 0 && perm[pivot - 1] > perm[pivot] { pivot -= 1; }
	if pivot == 0 { return false; }
	while perm[succ] < perm[pivot - 1] { succ -= 1; }
	perm.swap(pivot - 1, succ);
	perm[pivot..].reverse();
	true
}

fn main() {
	let (prog, mut progs, mut phases_a, mut phases_b, mut max_a, mut max_b) = (read_to_string("in7.txt").unwrap().trim_end().split(',').map(|int| int.parse::<i32>().unwrap()).collect::<Vec<_>>(), [vec![], vec![], vec![], vec![], vec![]], [0, 1, 2, 3, 4], [5, 6, 7, 8, 9], 0, 0);
	loop {
		let mut signal = 0;
		for amp in 0..5 {
			let (_, _, next_sig) = run(&mut prog.clone(), 0, false, phases_a[amp], signal);
			signal = next_sig.unwrap();
		}
		max_a = max(max_a, signal);
		for i in 0..5 { progs[i] = prog.clone(); }
		signal = 0;
		let (mut pos, mut has_in) = ([0; 5], [false; 5]);
		'fdbk: for _ in 0..1000 {
			for amp in 0..5 {
				let (next_pos, next_has, next_sig) = run(&mut progs[amp], pos[amp], has_in[amp], phases_b[amp], signal);
				if next_sig.is_none() { break 'fdbk; }
				pos[amp] = next_pos;
				has_in[amp] = next_has;
				signal = next_sig.unwrap();
			}
			max_b = max(max_b, signal);
		}
		if !next_perm(&mut phases_a) || !next_perm(&mut phases_b) { break; }
	}
	println!("Part A: {}", max_a); // 21860
	println!("Part B: {}", max_b); // 2645740
}

