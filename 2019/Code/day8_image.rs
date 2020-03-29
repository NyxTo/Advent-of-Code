use std::fs::read_to_string;

fn main() {
	let (img, mut min_n0, mut num_1x2, mut render) = (read_to_string("in8.txt").unwrap(), 151, 0, [[2; 25]; 6]);
	let mut digits = img.chars();
	for _ in 0 .. img.len() / 150 {
		let mut nums = [0; 3];
		for y in 0..6 {
		for x in 0..25 {
			let dig = digits.next().unwrap().to_digit(10).unwrap();
			nums[dig as usize] += 1;
			if render[y][x] == 2 { render[y][x] = dig; }
		}
		}
		if nums[0] < min_n0 {
			min_n0 = nums[0];
			num_1x2 = nums[1] * nums[2];
		}
	}
	println!("Part A: {}", num_1x2); // 2016
	println!("Part B:");
	for y in 0..6 {							//  #     #   # # # #     # #     # # # #   #     #
		for x in 0..25 { print!(" {}", match render[y][x] {	//  #     #         #   #     #         #   #     #
			0 => ' ',					//  # # # #       #     #             #     #     #
			1 => '#',					//  #     #     #       #           #       #     #
			_ => panic!("missing pixel"),                   //  #     #   #         #     #   #         #     #
		}); }							//  #     #   # # # #     # #     # # # #     # #
		println!();
	}
}
