use std::fs::read_to_string;
use std::cmp::max;

fn param(prog: &Vec<i32>, pos: usize, i: usize) -> i32 {
	match prog[pos] / 10_i32.pow(i as u32 + 1) % 10 {
		0 => prog[prog[pos + i] as usize],
		1 => prog[pos + i],
		_ => panic!("invalid parameter mode"),
	}
}

struct Amp {
	prog: Vec<i32>, pos: usize, has_in: bool, phase: i32, sig: i32,
}

impl Iterator for Amp {
	type Item = i32;
	fn next(&mut self) -> Option<i32> {
		loop {
			let op = self.prog[self.pos];
			if op == 99 { return None; }
			match op % 100 {
				1 => {
					let store = self.prog[self.pos + 3] as usize;
					self.prog[store] = param(&self.prog, self.pos, 1) + param(&self.prog, self.pos, 2);
					self.pos += 4;
				},
				2 => {
					let store = self.prog[self.pos + 3] as usize;
					self.prog[store] = param(&self.prog, self.pos, 1) * param(&self.prog, self.pos, 2);
					self.pos += 4;
				},
				3 => {
					let store = self.prog[self.pos + 1] as usize;
					self.prog[store] = if !self.has_in { self.phase } else { self.sig };
					self.has_in = true;
					self.pos += 2;
				},
				4 => {
					let output = param(&self.prog, self.pos, 1);
					self.pos += 2;
					return Some(output);
				},
				5 => self.pos = if param(&self.prog, self.pos, 1) != 0 { param(&self.prog, self.pos, 2) as usize } else { self.pos + 3 },
				6 => self.pos = if param(&self.prog, self.pos, 1) == 0 { param(&self.prog, self.pos, 2) as usize } else { self.pos + 3 },
				7 => {
					let store = self.prog[self.pos + 3] as usize;
					self.prog[store] = if param(&self.prog, self.pos, 1) < param(&self.prog, self.pos, 2) { 1 } else { 0 };
					self.pos += 4;
				},
				8 => {
					let store = self.prog[self.pos + 3] as usize;
					self.prog[store] = if param(&self.prog, self.pos, 1) == param(&self.prog, self.pos, 2) { 1 } else { 0 };
					self.pos += 4;
				},
				_ => panic!("invalid opcode"),
			}
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
	let (prog, mut amps, mut phases_a, mut phases_b, mut max_a, mut max_b) = (read_to_string("in7.txt").unwrap().trim_end().split(',').map(|int| int.parse::<i32>().unwrap()).collect::<Vec<_>>(), vec![], [0, 1, 2, 3, 4], [5, 6, 7, 8, 9], 0, 0);
	for _ in 0..5 { amps.push(Amp { prog: vec![], pos: 0, has_in: false, phase: 0, sig: 0 }); }
	loop {
		for i in 0..5 {
			amps[i].prog = prog.clone();
			amps[i].pos = 0;
			amps[i].has_in = false;
			amps[i].phase = phases_a[i];
		}
		let mut signal = 0;
		for i in 0..5 {
			amps[i].sig = signal;
			signal = amps[i].next().unwrap();
		}
		max_a = max(max_a, signal);
		for i in 0..5 {
			amps[i].prog = prog.clone();
			amps[i].pos = 0;
			amps[i].has_in = false;
			amps[i].phase = phases_b[i];
		}
		signal = 0;
		'fdbk: loop {
			for i in 0..5 {
				amps[i].sig = signal;
				let next_sig = amps[i].next();
				if next_sig.is_none() { break 'fdbk; }
				signal = next_sig.unwrap();
			}
			max_b = max(max_b, signal);
		}
		if !next_perm(&mut phases_a) || !next_perm(&mut phases_b) { break; }
	}
	println!("Part A: {}", max_a); // 21860
	println!("Part B: {}", max_b); // 2645740
}
