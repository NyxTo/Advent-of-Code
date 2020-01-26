fn main() {
	let (mut cnt_a, mut cnt_b) = (0, 0);
	'srch: for pswd in 206938..=679128 {
		let (mut num, mut prev_dig, mut run, mut has_grp, mut has_pair) = (pswd, 10, 0, false, false);
		let mut next_dig;
		while num > 0 {
			next_dig = prev_dig;
			prev_dig = num % 10;
			if prev_dig > next_dig { continue 'srch; }
			if prev_dig == next_dig { run += 1; }
			else {
				if run >= 2 { has_grp = true; }
				if run == 2 { has_pair = true; }
				run = 1;
			}
			num /= 10;
		}
		if has_grp || run >= 2{ cnt_a += 1; }
		if has_pair || run == 2 { cnt_b += 1; }
	}
	println!("Part A: {}", cnt_a); // 1653
	println!("Part B: {}", cnt_b); // 1133
}
