use std::fs::read_to_string;
use std::cmp::{min, max};

fn main() {
	let input = read_to_string("in3.txt").unwrap().trim_end().to_string();
	let sep = input.find('\n').unwrap();
	let (mut coord1, mut coord2, mut steps1, mut steps2) = (vec![0, 0], vec![0, 0], vec![0], vec![0]);
	for segment in (&input[..sep]).split(',') {
		let amt = segment.split_at(1).1.parse::<i32>().unwrap();
		coord1.push(coord1[coord1.len() - 2] + match segment.chars().next().unwrap() {
			'U' | 'R' => amt,
			'D' | 'L' => -amt,
			_ => panic!("fail"),
		});
		steps1.push(steps1[steps1.len() - 1] + amt);
	}
	for segment in (&input[sep+1 ..]).split(',') {
		let amt = segment.split_at(1).1.parse::<i32>().unwrap();
		coord2.push(coord2[coord2.len() - 2] + match segment.chars().next().unwrap() {
			'U' | 'R' => amt,
			'D' | 'L' => -amt,
			_ => panic!("fail"),
		});
		steps2.push(steps2[steps2.len() - 1] + amt);
	}
	let (mut min_dist, mut min_steps) = (300000, 300000);
	for i in 2 .. coord1.len() {
	for j in 2 .. coord2.len() {
		let (i2, i1, i0, j2, j1, j0) = (coord1[i-2], coord1[i-1], coord1[i], coord2[j-2], coord2[j-1], coord2[j]);
		if (i + j) % 2 == 1 && min(j2, j0) <= i1 && i1 <= max(j2, j0) && min(i2, i0) <= j1 && j1 <= max(i2, i0) {
			min_dist = min(min_dist, i1.abs() + j1.abs());
			min_steps = min(min_steps, steps1[i-2] + (i1 - j2).abs() + steps2[j-2] + (j1 - i2).abs());
		}
	}
	}
	println!("Part A: {}", min_dist);
	println!("Part B: {}", min_steps);
}
