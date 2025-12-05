use itertools::Itertools;

fn intersects(r1: &(u64, u64), r2: &(u64, u64)) -> bool {
	r1.1 >= r2.0 && r1.0 <= r2.1
}

pub fn solve(inputs: Vec<String>) {
	let (fresh_ranges, ingredients) = inputs
		.split(|line| line.is_empty())
		.collect_tuple()
		.unwrap();
	let ingredients = ingredients
		.iter()
		.map(|str| str.parse::<u64>().unwrap())
		.collect_vec();

	let mut fresh_ranges = fresh_ranges
		.iter()
		.map(|line| {
			let (start, end) = line.split_once("-").unwrap();
			(
				start.parse::<u64>().unwrap(),
				end.parse::<u64>().unwrap() + 1,
			)
		})
		.collect_vec();

	loop {
		let mut new_ranges: Vec<(u64, u64)> = Vec::with_capacity(fresh_ranges.len());
		for range in &fresh_ranges {
			let mut did_merge = false;
			for x in 0..new_ranges.len() {
				if intersects(range, &new_ranges[x]) {
					new_ranges[x] = (
						(range.0).min(new_ranges[x].0),
						(range.1).max(new_ranges[x].1),
					);
					did_merge = true;
					break;
				}
			}
			if !did_merge {
				new_ranges.push(*range);
			}
		}

		if new_ranges.len() != fresh_ranges.len() {
			fresh_ranges = new_ranges;
		} else {
			break;
		}
	}

	let part1 = ingredients
		.iter()
		.filter(|&ingredient| {
			fresh_ranges
				.iter()
				.any(|range| *ingredient >= range.0 && *ingredient < range.1)
		})
		.count();
	println!("Part 1: {part1}");

	let part2 = fresh_ranges
		.iter()
		.map(|range| range.1 - range.0)
		.sum::<u64>();
	println!("Part 2: {part2}");
}
