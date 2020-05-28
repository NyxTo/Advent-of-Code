use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::{min, max};

fn gcd(mut a: usize, mut b: usize) -> usize {
	while b > 0 {
		let rem = a % b;
		a = b;
		b = rem;
	}
	a
}

fn main() {
	let grid = BufReader::new(File::open("in10.txt").unwrap()).lines().map(|row| row.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
	let (hgt, wid, mut max_det, mut vapour) = (grid.len(), grid[0].len(), 0, (0, 0));
	for sy in 0..hgt {
	for sx in 0..wid {
	if grid[sy][sx] == '#' {
		let (mut detect, mut ang, mut btwn, mut aster) = (0, vec![vec![0.; wid]; hgt], vec![vec![0; wid]; hgt], vec![]);
		for ey in 0..hgt {
		for ex in 0..wid {
		if grid[ey][ex] == '#' {
			if ex == sx && ey == sy { continue; }
			ang[ey][ex] = (ex as f64 - sx as f64).atan2(ey as f64 - sy as f64);
			let num = gcd(max(sx, ex) - min(sx, ex), max(sy, ey) - min(sy, ey));
			btwn[ey][ex] = (1..num).filter(|i| {
				let (bx, by) = ((sx * (num - i) + ex * i) / num, (sy * (num - i) + ey * i) / num);
				grid[by][bx] == '#'
			}).count();
			if btwn[ey][ex] == 0 { detect += 1; }
			else { continue; }
			aster.push((ex, ey));
		}
		}
		}
		if detect >= max_det {
			max_det = detect;
			aster.sort_by(|(x1, y1), (x2, y2)| ang[*y2][*x2].partial_cmp(&ang[*y1][*x1]).unwrap());
			aster.sort_by_key(|(x, y)| btwn[*y][*x]);
			if aster.len() >= 200 { vapour = aster[199]; }
		}
	}
	}
	}
	println!("Part A: {}", max_det); // 221
	println!("Part B: {}", vapour.0 * 100 + vapour.1); // 806
}
