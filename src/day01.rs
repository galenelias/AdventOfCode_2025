pub fn solve(inputs: Vec<String>) {
	let mut dial: i32 = 50;
	let mut part1 = 0;
	let mut part2 = 0;

	for input in inputs {
		let dir = input.chars().next().unwrap();
		let mut num = input[1..].parse::<i32>().unwrap();

		part2 += num / 100;
		num = num % 100;

		match dir {
			'L' => {
				for _ in 0..num {
					dial -= 1;
					if dial < 0 {
						dial += 100;
					}
					if dial == 0 {
						part2 += 1;
					}
				}
			}
			'R' => {
				for _ in 0..num {
					dial += 1;
					if dial >= 100 {
						dial -= 100;
					}
					if dial == 0 {
						part2 += 1;
					}
				}
			}
			_ => unreachable!(),
		}

		if dial == 0 {
			part1 += 1;
		}
	}

	println!("Part 1: {part1}");
	println!("Part 2: {part2}");
}
