use std::fs::read_to_string;

fn main() {
	let input = read_to_string("in2.txt").unwrap().trim_end().to_string();
	let ints = input.split(',').map(|int| int.parse::<u32>().unwrap()).collect::<Vec<_>>();
	for noun in 0..100 {
	for verb in 0..100 {
		let mut code = ints.clone();
		code[1] = noun; code[2] = verb;
		for pos in (0 .. code.len()).step_by(4) {
			let (val1, val2, at) = (code[code[pos + 1] as usize], code[code[pos + 2] as usize], code[pos + 3] as usize);
			code[at] = match code[pos] {
				99 => break,
				1  => val1 + val2,
				2  => val1 * val2,
				_  => return panic!("fail"),
			}
		}
		if noun == 12 && verb == 2 { println!("Part A: {}", code[0]); }
		if code[0] == 19690720 { println!("Part B: {}", 100 * noun + verb); }
	}
	}
}
