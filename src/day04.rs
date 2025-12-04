use itertools::Itertools;

fn count_adjacent(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
	let mut result = 0;

	for r in row.saturating_sub(1)..grid.len().min(row + 2) {
		for c in col.saturating_sub(1)..grid[r].len().min(col + 2) {
			if r == row && c == col {
				continue;
			}

			if grid[r][c] == '@' {
				result += 1;
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
