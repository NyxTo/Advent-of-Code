use std::fs::File;
use std::io::{BufRead, BufReader};

fn sign(a: i32) -> i32 {
	if a > 0 { 1 } else if a < 0 { -1 } else { 0 }
}

fn lcm(a: i64, b: i64) -> i64 {
	let (mut x, mut y) = (a, b);
	while y > 0 {
		let rem = x % y;
		x = y;
		y = rem;
	}
	a * b / x
}

fn main() {
	let (moons, mut num, mut pos, mut energy) = (BufReader::new(File::open("in_12.txt").unwrap()).lines().map(|line| line.unwrap()), 0, vec![], 0);
	for line in moons {
		let (x_sep, y_sep, z_sep) = (line.find('x').unwrap(), line.find('y').unwrap(), line.find('z').unwrap());
		num += 1;
		pos.push((line[x_sep + 2 .. y_sep - 2].parse::<i32>().unwrap(),
		          line[y_sep + 2 .. z_sep - 2].parse::<i32>().unwrap(),
		          line[z_sep + 2 .. line.len() - 1].parse::<i32>().unwrap()));
	}
	let (init_pos, mut vel, mut rep) = (pos.clone(), vec![(0, 0, 0); num], (0, 0, 0));
	for time in 1..=300000 {
		for i in 0..num {
		for j in 0..num {
			vel[i].0 += sign(pos[j].0 - pos[i].0);
			vel[i].1 += sign(pos[j].1 - pos[i].1);
			vel[i].2 += sign(pos[j].2 - pos[i].2);
		}
		}
		for i in 0..num {
			pos[i].0 += vel[i].0;
			pos[i].1 += vel[i].1;
			pos[i].2 += vel[i].2;
		}
		if time == 1000 {
			for i in 0..num { energy += (pos[i].0.abs() + pos[i].1.abs() + pos[i].2.abs()) * (vel[i].0.abs() + vel[i].1.abs() + vel[i].2.abs()); }
		}
		if rep.0 == 0 && (0..num).all(|i| pos[i].0 == init_pos[i].0 && vel[i].0 == 0) { rep.0 = time; }
		if rep.1 == 0 && (0..num).all(|i| pos[i].1 == init_pos[i].1 && vel[i].1 == 0) { rep.1 = time; }
		if rep.2 == 0 && (0..num).all(|i| pos[i].2 == init_pos[i].2 && vel[i].2 == 0) { rep.2 = time; }
	}
	println!("Part A: {}", energy); // 12644
	println!("Part B: {}", lcm(lcm(rep.0, rep.1), rep.2)); // 290314621566528
}
