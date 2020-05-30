use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn dfs(directs: &HashMap<String, Vec<String>>, ctr: &str, dist_com: i32) -> (i32, i32, i32) {
	let (mut tot_com, mut dist_you, mut dist_san) = (dist_com, -1, -1);
	if let Some(orbits) = directs.get(ctr) {
		for arnd in orbits {
			let (tot_arnd, d_you, d_san) = dfs(directs, arnd, dist_com + 1);
			tot_com += tot_arnd;
			if arnd == "YOU" { dist_you = 0; }
			if arnd == "SAN" { dist_san = 0; }
			if d_you >= 0 && d_san >= 0 {
				dist_you = d_you;
				dist_san = d_san;
			}
			else if d_you >= 0 { dist_you = d_you + 1; }
			else if d_san >= 0 { dist_san = d_san + 1; }
			
		}
	}
	(tot_com, dist_you, dist_san)
}

fn main() {
	let mut directs: HashMap<_, Vec<String>> = HashMap::new();
	for line in BufReader::new(File::open("in6.txt").unwrap()).lines().map(|line| line.unwrap()) {
		let sep = line.find(')').unwrap();
		let (ctr, arnd) = (line[0..sep].to_string(), line[sep + 1 ..].to_string());
		match directs.get_mut(&ctr) {
			Some(orbits) => orbits.push(arnd),
			None => { directs.insert(ctr, vec![arnd]); },
		}
	}
	let (tot_com, dist_you, dist_san) = dfs(&directs, "COM", 0);
	println!("Part A: {}", tot_com); // 160040
	println!("Part B: {}", dist_you + dist_san); // 373
}
