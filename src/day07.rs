use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

fn part1(grid: &Vec<Vec<char>>, start: (usize, usize)) {
	let mut seen = HashSet::new();
	let mut queue = VecDeque::new();
	let mut splitters = 0;

	queue.push_back(start);
	while !queue.is_empty() {
		let pos = queue.pop_front().unwrap();
		if pos.0 >= grid.len() || !seen.insert(pos) {
			continue;
		}

		match grid[pos.0][pos.1] {
			'.' => queue.push_back((pos.0 + 1, pos.1)),
			'^' => {
				queue.push_back((pos.0, pos.1 - 1));
				queue.push_back((pos.0, pos.1 + 1));
				splitters += 1;
			}
			_ => unreachable!(),
		}
	}

	println!("Part 1: {splitters}");
}

fn sub_part2(
	grid: &Vec<Vec<char>>,
	r: usize,
	c: usize,
	memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
	if r >= grid.len() {
		return 1; // Base case
	}

	if let Some(memo_result) = memo.get(&(r, c)) {
		return *memo_result;
	}

	let ch = grid[r][c];
	let result = if ch == '.' {
		sub_part2(grid, r + 1, c, memo)
	} else if ch == '^' {
		sub_part2(grid, r, c - 1, memo) + sub_part2(grid, r, c + 1, memo)
	} else {
		unreachable!("Hit a {} at {},{}", ch, r, c);
	};

	memo.insert((r, c), result);
	return result;
}

pub fn solve(inputs: Vec<String>) {
	let mut grid = inputs
		.iter()
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let start = grid
		.iter()
		.enumerate()
		.find_map(|(r, row)| {
			row.iter()
				.enumerate()
				.find_map(|(c, ch)| if *ch == 'S' { Some((r, c)) } else { None })
		})
		.unwrap();

	grid[start.0][start.1] = '.';

	part1(&grid, start);

	let part2 = sub_part2(&grid, start.0, start.1, &mut HashMap::new());
	println!("Part 2: {part2}");
}
