use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug)]
struct Machine {
	lights: Vec<bool>,
	buttons: Vec<Vec<usize>>,
	_joltages: Vec<usize>,
}

fn turn_on(machine: &Machine) -> usize {
	let mut queue = VecDeque::new();

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

pub fn solve(inputs: Vec<String>) {
	let machines = inputs
		.iter()
		.map(|input| {
			let (lights, rest) = input.split_once(' ').unwrap();
			let (buttons, joltages) = rest.split_once('{').unwrap();

			let lights = lights
				.trim_matches(['[', ']'])
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

			let _joltages = joltages
				.trim_end_matches('}')
				.split(',')
				.map(|num| num.parse::<usize>().unwrap())
				.collect_vec();

			Machine {
				lights,
				buttons,
				_joltages,
			}
		})
		.collect_vec();

	let mut part1 = 0;
	for machine in &machines {
		let presses = turn_on(machine);
		part1 += presses;
	}

	println!("Part 1: {part1}");

	// Part 2 is solved in Python in day10.py
}
