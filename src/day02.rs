use itertools::Itertools;

pub fn solve(inputs: Vec<String>) {
	let mut part1 = 0;
	let mut part2 = 0;

	for range in inputs[0].split(',') {
		let (lower_str, upper_str) = range.split_once('-').unwrap();
		let lower = lower_str.parse::<u64>().unwrap();
		let upper = upper_str.parse::<u64>().unwrap();

		for x in lower..=upper {
			let chars = x.to_string().chars().collect_vec();

			let mut part1_valid = true;
			let mut part2_valid = true;

			// Try all possible repeating patterns of length N
			for pattern_len in 1..=(chars.len() / 2) {
				// But string must be divisible by N
				if (chars.len() % pattern_len) != 0 {
					continue;
				}

				let num_repeats = chars.len() / pattern_len;

				let mut is_repeated = true;
				for r in 1..num_repeats {
					for i in 0..pattern_len {
						if chars[i] != chars[r * pattern_len + i] {
							is_repeated = false;
						}
					}
				}

				if is_repeated {
					part2_valid = false;
				}

				if is_repeated && num_repeats == 2 {
					part1_valid = false;
				}
			}

			if !part1_valid {
				part1 += x;
			}

			if !part2_valid {
				part2 += x;
			}
		}
	}

	println!("Part 1: {part1}");
	println!("Part 2: {part2}");
}
