use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Machine {
	lights: Vec<bool>,
	buttons: Vec<Vec<usize>>,
	joltages: Vec<usize>,
}

// fn sub_solve(lights: &Vec<bool>, presses: usize)

fn turn_on(machine: &Machine) -> usize {
	let mut queue = VecDeque::new();
	// let mut seen = HashSet::new();

	queue.push_back((0, vec![false; machine.lights.len()]));

	while !queue.is_empty() {
		let (presses, lights) = queue.pop_front().unwrap();

		if lights == machine.lights {
			return presses;
		}

		// Try pressing each button
		for button in &machine.buttons {
			let mut new_lights = lights.clone();
			for &wire in button {
				new_lights[wire] = !new_lights[wire];
			}
			queue.push_back((presses + 1, new_lights));
		}
	}

	unreachable!("Couldn't find solution");
}

fn configure_joltage(machine: &Machine) -> usize {
	let mut queue = VecDeque::new();
	let mut seen = HashSet::new();

	queue.push_back((0, vec![0; machine.joltages.len()]));

	while !queue.is_empty() {
		let (presses, joltages) = queue.pop_front().unwrap();

		if joltages == machine.joltages {
			return presses;
		}

		if !seen.insert(joltages.clone()) {
			continue;
		}

		// Try pressing each button
		let mut is_good = true;
		for button in &machine.buttons {
			let mut new_joltages = joltages.clone();
			for &wire in button {
				new_joltages[wire] += 1;
				if new_joltages[wire] > machine.joltages[wire] {
					is_good = false;
					break;
				}
			}
			if is_good {
				queue.push_back((presses + 1, new_joltages));
			}
		}
	}

	unreachable!("Couldn't find solution");
}

pub fn solve(inputs: Vec<String>) {
	let machines = inputs
		.iter()
		.map(|input| {
			let (lights, rest) = input.split_once(' ').unwrap();
			let (buttons, joltages) = rest.split_once('{').unwrap();

			let lights = lights
				.trim_matches(['[', ']'])
				// .trim_end_matches(']')
				.chars()
				.map(|ch| match ch {
					'.' => false,
					'#' => true,
					_ => unreachable!("Unexpected character '{}'", ch),
				})
				.collect_vec();

			let buttons = buttons
				.split_whitespace()
				.map(|button| {
					button
						.trim_matches(['(', ')'])
						.split(',')
						.map(|num| num.parse::<usize>().unwrap())
						.collect_vec()
				})
				.collect_vec();

			let joltages = joltages
				.trim_end_matches('}')
				.split(',')
				.map(|num| num.parse::<usize>().unwrap())
				.collect_vec();

			Machine {
				lights,
				buttons,
				joltages,
			}
		})
		.collect_vec();

	let mut part1 = 0;
	for machine in &machines {
		let presses = turn_on(machine);
		// println!("Solved in {presses} presses");
		part1 += presses;
	}

	// println!("{:?}", machines);
	println!("Part 1: {part1}");

	let mut part2 = 0;
	for machine in &machines {
		let presses = configure_joltage(machine);
		println!("Solved in {presses} presses");
		part2 += presses;
	}
	println!("Part 2: {part2}");
}
