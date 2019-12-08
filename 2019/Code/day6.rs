use std::fs::read_to_string;
use std::collections::HashMap;

fn dfs(orbits: &HashMap<&str, Vec<&str>>, ctr: &str, d_com: u32) -> (u32, i16, i16) {
	let (mut tot_com, mut d_you, mut d_san) = (d_com, -1, -1);
	if let Some(ters) = orbits.get(ctr) {
		for arnd in ters {
			let ter = dfs(orbits, arnd, d_com + 1);
			tot_com += ter.0;
			if *arnd == "YOU" { d_you = 0; }
			if *arnd == "SAN" { d_san = 0; }
			if ter.1 >= 0 && ter.2 >= 0 { d_you = ter.1; d_san = ter.2; }
			else if ter.1 >= 0 { d_you = ter.1 + 1; }
			else if ter.2 >= 0 { d_san = ter.2 + 1; }
		}
	}
	return (tot_com, d_you, d_san);
}

fn main() {
	let input = read_to_string("in6.txt").unwrap();
	let mut orbits: HashMap<&str, Vec<&str>> = HashMap::new();
	for line in input.lines() {
		let sep = line.find(')').unwrap();
		let (ctr, arnd) = (&line[0..sep], &line[sep+1 ..]);
		match orbits.get_mut(ctr) {
			Some(ters) => ters.push(arnd),
			None => { orbits.insert(ctr, vec![arnd]); },
		}
	}
	let com = dfs(&orbits, "COM", 0);
	println!("Part A: {}", com.0);
	println!("Part B: {}", com.1 + com.2);
}
