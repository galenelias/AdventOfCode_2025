use std::i64;

use itertools::Itertools;
use std::collections::HashMap;

fn dist2(pt1: &(i64, i64, i64), pt2: &(i64, i64, i64)) -> i64 {
	(pt2.0 - pt1.0).pow(2) + (pt2.1 - pt1.1).pow(2) + (pt2.2 - pt1.2).pow(2)
}

pub fn solve(inputs: Vec<String>) {
	let boxes = inputs
		.iter()
		.map(|input| -> (i64, i64, i64) {
			input
				.split(',')
				.map(|str| str.parse::<i64>().unwrap())
				.collect_tuple()
				.unwrap()
		})
		.collect_vec();

	let count = boxes.len();
	let mut distances = Vec::with_capacity(count);
	for i in 0..count {
		for j in i + 1..count {
			let dist = dist2(&boxes[i], &boxes[j]);
			distances.push((dist, i, j));
		}
	}

	distances.sort();

	let mut group_nums = vec![0; count];

	for x in 0.. {
		let d = &distances[x];

		let group_num = x + 1;
		let gn1 = group_nums[d.1];
		let gn2 = group_nums[d.2];

		if gn1 == 0 {
			group_nums[d.1] = group_num;
		} else {
			for g in 0..count {
				if group_nums[g] == gn1 {
					group_nums[g] = group_num;
				}
			}
		}

		if gn2 == 0 {
			group_nums[d.2] = group_num;
		} else {
			for g in 0..count {
				if group_nums[g] == gn2 {
					group_nums[g] = group_num;
				}
			}
		}

		// If we've hit the part 1 limit, print the three largest group sizes
		if x + 1 == 1000 {
			// Count the distinct group sizes
			let mut group_counts: HashMap<usize, i64> = HashMap::new();
			for &gn in &group_nums {
				if gn != 0 {
					*(group_counts.entry(gn).or_default()) += 1;
				}
			}

			let group_count_values = group_counts.values().cloned().sorted().rev().collect_vec();

			println!(
				"Part 1: {}",
				group_count_values.iter().take(3).product::<i64>()
			);
		}

		// Check if we've merged everything into one group!
		if group_nums.iter().all(|&gn| gn == group_num) {
			println!("Part 2: {}", boxes[d.1].0 * boxes[d.2].0);
			break;
		}
	}
}
