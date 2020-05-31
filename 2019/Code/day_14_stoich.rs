use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, VecDeque};

fn ore_nd(rxns: &HashMap<String, Vec<(String, i64)>>, coeffs: &HashMap<String, i64>, mut uses: HashMap<String, i32>, fuel: i64) -> i64 {
	let (mut kahn, mut need) = (VecDeque::new(), HashMap::new());
	kahn.push_back(String::from("FUEL"));
	need.insert(String::from("FUEL"), fuel);
	loop {
		let pdt = kahn.pop_front().unwrap();
		if pdt == "ORE" { return *need.get(&pdt).unwrap(); }
		let times = (need.get(&pdt).unwrap() - 1) / coeffs.get(&pdt).unwrap() + 1;
		for (chem, qty) in rxns.get(&pdt).unwrap() {
			*uses.get_mut(&chem.clone()).unwrap() -= 1;
			match need.get_mut(&chem.clone()) {
				Some(amt) => *amt += times * qty,
				None => { need.insert(chem.clone(), times * qty); },
			}
			if *uses.get(&chem.clone()).unwrap() == 0 { kahn.push_back(chem.to_string()); }
			if *uses.get(&chem.clone()).unwrap() < 0 { panic!("{} uses {}", chem, uses.get(&chem.clone()).unwrap()); }
			
		}
	}
}

fn main() {
	let (mut rxns, mut coeffs, mut uses) = (HashMap::new(), HashMap::new(), HashMap::new());
	for line in BufReader::new(File::open("in_14.txt").unwrap()).lines().map(|line| line.unwrap()) {
		let arrow = line.find("=>").unwrap();
		let rctnts = line[.. arrow - 1].split(", ").map(|rctnt| {
			let spc = rctnt.find(' ').unwrap();
			let (qty, chem) = (rctnt[..spc].parse::<i64>().unwrap(), rctnt[spc + 1 ..].to_string());
			match uses.get_mut(&chem) {
				Some(amt) => *amt += 1,
				None => { uses.insert(chem.clone(), 1); },
			}
			(chem, qty)
		}).collect::<Vec<_>>();
		let pdt = line[arrow + 3 ..].to_string();
		let spc = pdt.find(' ').unwrap();
		let (qty, chem) = (pdt[..spc].parse::<i64>().unwrap(), pdt[spc + 1 ..].to_string());
		rxns.insert(chem.clone(), rctnts);
		coeffs.insert(chem.clone(), qty);
	}
	println!("Part A: {}", ore_nd(&rxns, &coeffs, uses.clone(), 1)); // 397771
	let mut exp = 1;
	while ore_nd(&rxns, &coeffs, uses.clone(), 1 << exp) < 10_i64.pow(12) { exp += 1; }
	let (mut low_bd, mut up_bd) = (1 << (exp - 1), 1 << exp);
	while up_bd - low_bd > 1 {
		let mid = (low_bd + up_bd) / 2;
		if ore_nd(&rxns, &coeffs, uses.clone(), mid) < 10_i64.pow(12) { low_bd = mid; }
		else { up_bd = mid; }
	}
	println!("Part B: {}", low_bd); // 3126714
}
