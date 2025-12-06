use itertools::Itertools;

enum Op {
	Add,
	Mul,
}

fn part1(inputs: &[String], ops: &[Op]) {
	let grid = inputs
		.iter()
		.map(|line| {
			line.split_whitespace()
				.map(|tok| tok.parse::<i64>().unwrap())
				.collect_vec()
		})
		.collect_vec();
	let part1 = (0..grid[0].len())
		.map(|c| match ops[c] {
			Op::Add => (0..grid.len()).fold(0, |acc, r| acc + grid[r][c]),
			Op::Mul => (0..grid.len()).fold(1, |acc, r| acc * grid[r][c]),
		})
		.sum::<i64>();

	println!("Part 1: {part1}");
}

fn part2(inputs: &[String], ops: &[Op]) {
	let grid = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let mut part2 = 0;
	let mut col_num = 0;
	let mut prev_col = 0;
	let max_line_len = grid.iter().map(|line| line.len()).max().unwrap();

	for c in 0..max_line_len + 1 {
		if !(0..grid.len()).all(|r| c >= grid[r].len() || grid[r][c] == ' ') {
			continue;
		}

		let mut result = match ops[col_num] {
			Op::Add => 0,
			Op::Mul => 1,
		};

		for dc in (prev_col..c).rev() {
			let mut digits = Vec::new();
			for r in 0..grid.len() {
				if dc < grid[r].len() && grid[r][dc] != ' ' {
					digits.push(grid[r][dc]);
				}
			}

			if !digits.is_empty() {
				let num: String = digits.iter().collect();
				let num = num.parse::<i64>().unwrap();
				result = match ops[col_num] {
					Op::Add => result + num,
					Op::Mul => result * num,
				};
			}
		}
		part2 += result;
		col_num += 1;
		prev_col = c + 1;
	}

	println!("Part 2: {part2}");
}

pub fn solve(inputs: Vec<String>) {
	let numbers = &inputs[0..inputs.len() - 1];
	let ops = inputs
		.last()
		.unwrap()
		.split_whitespace()
		.map(|ch| match ch {
			"+" => Op::Add,
			"*" => Op::Mul,
			_ => unreachable!(),
		})
		.collect_vec();

	part1(numbers, &ops);
	part2(numbers, &ops);
}
