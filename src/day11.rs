use itertools::Itertools;
use std::collections::HashMap;

fn sub_solve(
	devices: &HashMap<String, Vec<String>>,
	start: &str,
	end: &str,
	omit: &str,
	memo: &mut HashMap<String, usize>,
) -> usize {
	if start == end {
		return 1;
	}

	if start == omit || start == "out" {
		return 0;
	}

	if let Some(mem) = memo.get(start) {
		return *mem;
	}

	let result = devices
		.get(start)
		.unwrap()
		.iter()
		.map(|output| sub_solve(devices, output, end, omit, memo))
		.sum::<usize>();
	memo.insert(start.to_owned(), result);
	return result;
}

pub fn solve(inputs: Vec<String>) {
	let devices: HashMap<String, Vec<String>> = inputs
		.iter()
		.map(|input| {
			let (name, outputs) = input.split_once(": ").unwrap();

			let outputs = outputs
				.split_whitespace()
				.map(|str| str.to_owned())
				.collect_vec();

			(name.to_owned(), outputs)
		})
		.collect();

	let part1 = sub_solve(&devices, "you", "out", "", &mut HashMap::new());
	println!("Part 1: {part1}");

	let svr_to_dac = sub_solve(&devices, "svr", "dac", "fft", &mut HashMap::new());
	let dac_to_fft = sub_solve(&devices, "dac", "fft", "", &mut HashMap::new());
	let fft_to_out = sub_solve(&devices, "fft", "out", "dac", &mut HashMap::new());

	let svr_to_fft = sub_solve(&devices, "svr", "fft", "dac", &mut HashMap::new());
	let fft_to_dac = sub_solve(&devices, "fft", "dac", "", &mut HashMap::new());
	let dac_to_out = sub_solve(&devices, "dac", "out", "fft", &mut HashMap::new());

	let part2 = svr_to_dac * dac_to_fft * fft_to_out + svr_to_fft * fft_to_dac * dac_to_out;
	println!("Part 2: {part2}");
}
