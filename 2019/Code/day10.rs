use std::fs::read_to_string;
use std::cmp::{min, max};

fn gcd(mut a: usize, mut b: usize) -> usize {
	while b != 0 {
		let r = a % b;
		a = b;
		b = r;
	}
	return a;
}

fn main() {
	let input = read_to_string("in10.txt").unwrap();
	let grid = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
	let (mut max_det, mut vapour) = (0, (0, 0));
	for sy in 0 .. grid.len() {
	for sx in 0 .. grid[sy].len() {
	if grid[sy][sx] == '#' {
		let mut detect = vec![];
		for ey in 0 .. grid.len() {
		'search:
		for ex in 0 .. grid[ey].len() {
		if !(ex == sx && ey == sy) && grid[ey][ex] == '#' {
			let d = gcd(max(sx, ex) - min(sx, ex), max(sy, ey) - min(sy, ey));
			for i in 1..d {
				let (mx, my) = (((d - i) * sx + i * ex) / d, ((d - i) * sy + i * ey) / d);
				if grid[my][mx] == '#' { continue 'search; }
			}
			detect.push((ex, ey));
		}
		}
		}
		if detect.len() >= max_det {
			max_det = detect.len();
			detect.sort_by(|(x1, y1), (x2, y2)| {
				let a1 = (*x1 as f64 - sx as f64).atan2(*y1 as f64 - sy as f64);
				let a2 = (*x2 as f64 - sx as f64).atan2(*y2 as f64 - sy as f64);
				a2.partial_cmp(&a1).unwrap()
			});
			max_det = max(max_det, detect.len());
			if detect.len() >= 200 { vapour = detect[199]; }
		}
	}
	}
	}
	println!("Part A: {}", max_det);
	println!("Part B: {}", vapour.0 * 100 + vapour.1);
}
