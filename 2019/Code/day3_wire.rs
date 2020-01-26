use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::{min, max};

fn path(wire: Vec<(char, i32)>) -> (Vec<i32>, Vec<i32>) {
	let (mut coords, mut steps) = (vec![0, 0], vec![0]);
	for (dir, amt) in wire {
		coords.push(coords[coords.len() - 2] + match dir {
			'U' | 'R' => amt,
			'D' | 'L' => -amt,
			_ => panic!("invalid direction"),
		});
		steps.push(steps[steps.len() - 1] + amt);
	}
	(coords, steps)
}

fn main() {
	let mut input = BufReader::new(File::open("in3.txt").unwrap()).lines().map(|line| line.unwrap().split(',').map(|segment| (segment.chars().next().unwrap(), segment[1..].parse::<i32>().unwrap())).collect::<Vec<_>>());
	let ((coords1, steps1), (coords2, steps2), mut min_dist, mut min_steps) = (path(input.next().unwrap()), path(input.next().unwrap()), 90000000, 90000000);
	for i in 0 .. coords1.len() - 2 {
	for j in 0 .. coords2.len() - 2 {
		let (ci0, ci1, ci2, cj0, cj1, cj2) = (coords1[i], coords1[i + 1], coords1[i + 2], coords2[j], coords2[j + 1], coords2[j + 2]);
		if (i + j) % 2 == 1 && min(cj0, cj2) <= ci1 && ci1 <= max(cj0, cj2) && min(ci0, ci2) <= cj1 && cj1 <= max(ci0, ci2) {
			min_dist = min(min_dist, ci1.abs() + cj1.abs());
			min_steps = min(min_steps, steps1[i] + (ci1 - cj0).abs() + steps2[j] + (cj1 - ci0).abs());
		}
	}
	}
	println!("Part A: {}", min_dist); // 1337
	println!("Part B: {}", min_steps); // 65356
}
