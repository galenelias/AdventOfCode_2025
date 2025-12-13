use itertools::Itertools;

const SHAPE_SIZE: usize = 3;
type Shape = [[bool; 3]; 3];

fn rotate(input: &Shape) -> Shape {
	let mut output = input.clone();
	let ln = input.len();
	for r in 0..ln {
		for c in 0..ln {
			output[c][ln - 1 - r] = input[r][c];
		}
	}
	output
}

fn flip(input: &Shape) -> Shape {
	let mut output = input.clone();
	let ln = input.len();
	for r in 0..ln {
		for c in 0..ln {
			output[r][ln - 1 - c] = input[r][c];
		}
	}
	output
}

fn permute_shapes(shapes: &Vec<Shape>) -> Vec<Vec<Shape>> {
	shapes
		.iter()
		.map(|shape| {
			let mut all_permutations = Vec::new();
			let mut temp = shape.clone();
			for _ in 0..4 {
				temp = rotate(&temp);
				all_permutations.push(temp.clone());
				all_permutations.push(flip(&temp));
			}

			all_permutations.into_iter().unique().collect_vec()
		})
		.collect_vec()
}

fn count_squares(shape: &Shape) -> usize {
	shape.iter().flatten().filter(|cell| **cell).count()
}

fn can_fit_shapes(
	shapes: &Vec<Vec<Shape>>,
	width: usize,
	height: usize,
	shape_reqs: &Vec<usize>,
) -> bool {
	let rows = height / SHAPE_SIZE;
	let cols = width / SHAPE_SIZE;
	let total_shapes = shape_reqs.iter().sum::<usize>();
	let total_shape_squares = (0..shapes.len())
		.map(|i| count_squares(&shapes[i][0]) * shape_reqs[i])
		.sum::<usize>();

	if rows * cols >= total_shapes {
		return true;
	} else if total_shape_squares > height * width {
		return false;
	} else {
		let mut grid = vec![vec![false; width]; height];
		let mut shape_reqs = shape_reqs.clone();
		return can_fit_shapes_sub(shapes, &mut grid, &mut shape_reqs);
	}
}

fn shape_fits_in_grid(grid: &Vec<Vec<bool>>, shape: &Shape, gr: usize, gc: usize) -> bool {
	for r in 0..shape.len() {
		for c in 0..shape[r].len() {
			if shape[r][c] && grid[gr + r][gc + c] {
				return false;
			}
		}
	}
	return true;
}

fn toggle_shape_bits_in_grid(grid: &mut Vec<Vec<bool>>, shape: &Shape, gr: usize, gc: usize) {
	for r in 0..shape.len() {
		for c in 0..shape[r].len() {
			if shape[r][c] {
				grid[gr + r][gc + c] = !grid[gr + r][gc + c];
			}
		}
	}
}

fn can_fit_shapes_sub(
	shapes: &Vec<Vec<Shape>>,
	grid: &mut Vec<Vec<bool>>,
	shape_reqs: &mut Vec<usize>,
) -> bool {
	if shape_reqs.iter().all(|&val| val == 0) {
		return true;
	}

	let (shape_num, _) = shape_reqs
		.iter()
		.enumerate()
		.filter(|(_, num)| **num > 0)
		.next()
		.unwrap();

	for permutation in &shapes[shape_num] {
		for r in 0..=(grid.len() - permutation.len()) {
			for c in 0..=(grid[r].len() - permutation[0].len()) {
				if shape_fits_in_grid(grid, permutation, r, c) {
					toggle_shape_bits_in_grid(grid, permutation, r, c);
					shape_reqs[shape_num] -= 1;

					// Recurse
					let result = can_fit_shapes_sub(shapes, grid, shape_reqs);
					if result {
						return true;
					}

					toggle_shape_bits_in_grid(grid, permutation, r, c);
					shape_reqs[shape_num] += 1;
				}
			}
		}
	}
	false
}

pub fn solve(inputs: Vec<String>) {
	let parts = inputs.split(|line| line.is_empty()).collect_vec();

	let shapes = parts[0..(parts.len() - 1)]
		.iter()
		.map(|shape| {
			shape[1..]
				.iter()
				.map(|line| {
					line.chars()
						.map(|ch| match ch {
							'.' => false,
							'#' => true,
							_ => unreachable!(),
						})
						.collect_array()
						.unwrap()
				})
				.collect_array()
				.unwrap()
		})
		.collect_vec();

	// Expand shapes into all possible permuations
	let shape_permutations = permute_shapes(&shapes);
	let regions = parts.last().unwrap();

	let mut part1 = 0;
	for region in *regions {
		let (size, presents) = region.split_once(": ").unwrap();
		let presents = presents
			.split_whitespace()
			.map(|str| str.parse::<usize>().unwrap())
			.collect_vec();

		let (width, height) = size
			.split('x')
			.map(|str| str.parse::<usize>().unwrap())
			.collect_tuple()
			.unwrap();

		let result = can_fit_shapes(&shape_permutations, width, height, &presents);
		if result {
			part1 += 1;
		}
	}
	println!("Part 1: {part1}");
}
