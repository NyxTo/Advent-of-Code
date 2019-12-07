use std::fs::read_to_string;

fn program(mut code: Vec<i32>, id: i32) -> i32 {
	let (mut pos, mut has_in, mut has_out, mut diagnostic) = (0, false, false, -1);
	loop {
		let op = code[pos];
		if op == 99 { break; }
		let mode = |pow10, val| if op / pow10 % 10 == 0 { code[val as usize] } else { val };
		let (mut val1, mut val2, at) = (code[pos + 1], code[pos + 2], code[pos + 3] as usize);
		match op % 100 {
			1 | 2 => {
				val1 = mode(100, val1);
				val2 = mode(1000, val2);
				code[at] = if op % 100 == 1 { val1 + val2 } else { val1 * val2 };
				pos += 4;
			},
			3 => {
				if has_in { panic!("fail"); }
				code[val1 as usize] = id;
				has_in = true;
				pos += 2;
			},
			4 => {
				if has_out { panic!("fail"); }
				val1 = mode(100, val1);
				if val1 != 0 {
					diagnostic = val1;
					has_out = true;
				}
				pos += 2;
			},
			5 | 6 => {
				if (op % 100 == 5) ^ (mode(100, val1) == 0) { pos = mode(1000, val2) as usize; }
				else { pos += 3; }
			},
			7 => {
				code[at] = if mode(100, val1) < mode(1000, val2) { 1 } else { 0 };
				pos += 4;
			},
			8 => {
				code[at] = if mode(100, val1) == mode(1000, val2) { 1 } else { 0 };
				pos += 4;
			},
			_ => panic!("fail"),
		}
	}
	return diagnostic;
}

fn main() {
	let input = read_to_string("in5.txt").unwrap().trim_end().to_string();
	let code = input.split(",").map(|int| int.parse::<i32>().unwrap()).collect::<Vec<_>>();
	println!("Part A: {}", program(code.clone(), 1));
	println!("Part B: {}", program(code.clone(), 5));
}
