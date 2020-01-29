use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let (mut sum_a, mut sum_b) = (0, 0);
	for mass in BufReader::new(File::open("in1.txt").unwrap()).lines().map(|line| line.unwrap().parse::<i32>().unwrap()) {
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
