use std::fs::read_to_string;

fn main() {
	let input = read_to_string("in1.txt").unwrap();
	let (mut sum_a, mut sum_b) = (0, 0);
	for line in input.lines() {
		let mass = line.parse::<i32>().unwrap();
		let mut fuel = mass / 3 - 2;
		sum_a += fuel;
		while fuel > 0 {
			sum_b += fuel;
			fuel = fuel / 3 - 2;
		}
	}
	println!("Part A: {}", sum_a);
	println!("Part B: {}", sum_b);
}
