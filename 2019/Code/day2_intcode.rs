use std::fs::read_to_string;

fn main() {
	for noun in 0..100 {
	for verb in 0..100 {
		let (mut prog, mut pos) = (read_to_string("in2.txt").unwrap().trim_end().split(',').map(|int| int.parse::<i32>().unwrap()).collect::<Vec<_>>(), 0);
		prog[1] = noun;
		prog[2] = verb;
		loop {
			let store = prog[pos + 3] as usize;
			match prog[pos] {
				1 => prog[store] = prog[prog[pos + 1] as usize] + prog[prog[pos + 2] as usize],
				2 => prog[store] = prog[prog[pos + 1] as usize] * prog[prog[pos + 2] as usize],
				99 => break,
				_ => panic!("unknown opcode"),
			};
			pos += 4;
		}
		if noun == 12 && verb == 2 { println!("Part A: {}", prog[0]); } // 3895705
		if prog[0] == 19690720 { println!("Part B: {}", 100 * noun + verb); } // 6417
	}
	}
}
