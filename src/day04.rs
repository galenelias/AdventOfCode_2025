use itertools::Itertools;

fn count_adjacent(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
	let mut result = 0;

	for dr in -1..=1 {
		if let Some(r) = row.checked_add_signed(dr) {
			if r >= grid.len() {
				continue;
			}

			for dc in -1..=1 {
				if let Some(c) = col.checked_add_signed(dc) {
					if c >= grid[r].len() {
						continue;
					}

					if dr == 0 && dc == 0 {
						continue;
					}

					if grid[r][c] == '@' {
						result += 1;
					}
				}
			}
		}
	}
	return result;
}

fn scan_paper(grid: &mut Vec<Vec<char>>, should_remove: bool) -> usize {
	let mut could_remove = 0;
	for r in 0..grid.len() {
		for c in 0..grid.len() {
			if grid[r][c] == '@' && count_adjacent(&grid, r, c) < 4 {
				if should_remove {
					grid[r][c] = '.';
				}
				could_remove += 1;
			}
		}
	}
	return could_remove;
}

pub fn solve(inputs: Vec<String>) {
	let mut grid = inputs
		.iter()
		.map(|input| input.chars().collect_vec())
		.collect_vec();

	let part1 = scan_paper(&mut grid.clone(), false);
	println!("Part 1: {part1}");

	let mut part2 = 0;
	loop {
		let removed = scan_paper(&mut grid, true);
		part2 += removed;
		if removed == 0 {
			break;
		}
	}
	println!("Part 2: {part2}");
}
