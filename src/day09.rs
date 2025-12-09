use itertools::Itertools;

pub fn solve(inputs: Vec<String>) {
	let corners = inputs
		.iter()
		.map(|input| -> (i64, i64) {
			input
				.split(',')
				.map(|str| str.parse::<i64>().unwrap())
				.collect_tuple()
				.unwrap()
		})
		.collect_vec();

	let mut part1 = 0;
	let mut part2 = 0;

	for i in 0..corners.len() {
		for j in (i + 1)..corners.len() {
			let pt1 = (
				corners[i].0.min(corners[j].0),
				corners[i].1.min(corners[j].1),
			);
			let pt2 = (
				corners[i].0.max(corners[j].0),
				corners[i].1.max(corners[j].1),
			);
			let area = (pt2.0 - pt1.0 + 1) * (pt2.1 - pt1.1 + 1);

			if area > part1 {
				part1 = area;
			}

			if area > part2 {
				// Check each edge to see if it intersects with our candidate rectangle.
				// If any edge runs perpendicularly, then it means part of our rectangle is outside the polygon.
				let mut any_edge_intersects = false;
				for k1 in 0..corners.len() {
					let k2 = (k1 + 1) % corners.len();
					let e1 = (
						corners[k1].0.min(corners[k2].0),
						corners[k1].1.min(corners[k2].1),
					);
					let e2 = (
						corners[k1].0.max(corners[k2].0),
						corners[k1].1.max(corners[k2].1),
					);

					if (e1.0 == e2.0 // Vertical line
						&& e1.0 > pt1.0 && e1.0 < pt2.0
						&& e1.1 < pt2.1 && e2.1 > pt1.1)
						|| (e1.1 == e2.1 // Horizontal line
							&& e1.1 > pt1.1 && e1.1 < pt2.1
							&& e1.0 < pt2.0 && e2.0 > pt1.0)
					{
						any_edge_intersects = true;
						break;
					}
				}

				if !any_edge_intersects {
					part2 = area;
				}
			}
		}
	}

	println!("Part 1: {part1}");
	println!("Part 2: {part2}");
}
