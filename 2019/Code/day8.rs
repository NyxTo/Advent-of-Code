use std::fs::read_to_string;

fn main() {
	let input = read_to_string("in8.txt").unwrap();
	let (mut digits, mut min_n0, mut mult_12, mut img) = (input.chars(), 151, 0, [[2; 25]; 6]);
	for _ in 0 .. input.len() / 150 {
		let (mut n0, mut n1, mut n2) = (0, 0, 0);
		for pxl in 0..150 {
			let (x, y) = (pxl % 25, pxl / 25);
			match digits.next().unwrap().to_digit(10).unwrap() {
				0 => {
					n0 += 1;
					if img[y][x] == 2 { img[y][x] = 0; }
				},
				1 => {
					n1 += 1;
					if img[y][x] == 2 { img[y][x] = 1; }
				},
				2 => n2 += 1,
				_ => panic!("fail"),
			}
		}
		if n0 < min_n0 {
			min_n0 = n0;
			mult_12 = n1 * n2;
		}
	}
	println!("Part A: {}", mult_12);
	println!("Part B:");
	for y in 0..6 {
		for x in 0..25 {
			print!(" {}", match img[y][x] {
				0 => ' ',
				1 => '#',
				_ => panic!("fail"),
			});
		}
		println!();
	}
}
