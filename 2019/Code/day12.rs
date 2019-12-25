use std::fs::read_to_string;

fn sign(a: i16) -> i16 {
	if a > 0 { 1 } else if a < 0 { -1 } else { 0 }
}

fn lcm(a: u64, b: u64) -> u64 {
	let (mut x, mut y) = (a, b);
	while y > 0 {
		let r = x % y;
		x = y;
		y = r;
	}
	a * b / x
}

fn main() {
	let input = read_to_string("in12.txt").unwrap();
	let num = input.lines().count();
	let (mut i_px, mut i_py, mut i_pz, mut vx, mut vy, mut vz) = (vec![], vec![], vec![], vec![0; num], vec![0; num], vec![0; num]);
	for line in input.lines() {
		let (xsep, ysep, zsep) = (line.find('x').unwrap(), line.find('y').unwrap(), line.find('z').unwrap());
		i_px.push(line[xsep+2 .. ysep-2].parse::<i16>().unwrap());
		i_py.push(line[ysep+2 .. zsep-2].parse::<i16>().unwrap());
		i_pz.push(line[zsep+2 .. line.len()-1].parse::<i16>().unwrap());
	}
	let (mut px, mut py, mut pz, mut rep_x, mut rep_y, mut rep_z, mut step) = (i_px.clone(), i_py.clone(), i_pz.clone(), 0, 0, 0, 1);
	let (mut f_px, mut f_py, mut f_pz, mut f_vx, mut f_vy, mut f_vz) = (vec![], vec![], vec![], vec![], vec![], vec![]);
	while step <= 1000 || rep_x == 0 || rep_y == 0 || rep_z == 0 {
		for i in 0..num {
		for j in 0..num {
			vx[i] += sign(px[j] - px[i]);
			vy[i] += sign(py[j] - py[i]);
			vz[i] += sign(pz[j] - pz[i]);
		}
		}
		for i in 0..num {
			px[i] += vx[i];
			py[i] += vy[i];
			pz[i] += vz[i];
		}
		if step == 1000 {
			f_px = px.clone();
			f_py = py.clone();
			f_pz = pz.clone();
			f_vx = vx.clone();
			f_vy = vy.clone();
			f_vz = vz.clone();
		}
		if rep_x == 0 && px == i_px && vx == vec![0; num] { rep_x = step; }
		if rep_y == 0 && py == i_py && vy == vec![0; num] { rep_y = step; }
		if rep_z == 0 && pz == i_pz && vz == vec![0; num] { rep_z = step; }
		step += 1;
	}
	let mut tot = 0;
	for i in 0..num {
		tot += (f_px[i].abs() + f_py[i].abs() + f_pz[i].abs()) * (f_vx[i].abs() + f_vy[i].abs() + f_vz[i].abs());
	}
	println!("Part A: {}", tot);
	println!("Part B: {}", lcm(lcm(rep_x, rep_y), rep_z));
}
