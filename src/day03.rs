fn sub_solve(batteries: &Vec<Vec<u64>>, activate_num: usize) -> u64 {
	batteries.iter().map(|row| {
		let mut joltage = 0;
		let mut next_index = 0;
		for activate_index in 0..activate_num {
			let mut max_index = next_index;
			let remaining = activate_num - activate_index - 1;
			for i in (next_index + 1)..(row.len() - remaining) {
				if row[i] > row[max_index] {
					max_index = i;
				}
			}

			joltage = (joltage * 10) + row[max_index];
			next_index = max_index + 1;
		}

		joltage
	}).sum::<u64>()
}

pub fn solve(inputs: Vec<String>) {
	let batteries: Vec<Vec<u64>> = inputs.iter().map(|input| input.chars().map(|c| c.to_digit(10).unwrap() as u64).collect()).collect();
	println!("Part 1: {}", sub_solve(&batteries, 2));
	println!("Part 2: {}", sub_solve(&batteries, 12));
}
