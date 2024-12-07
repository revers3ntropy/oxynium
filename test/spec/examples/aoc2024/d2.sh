describe 'AOC 2024 Day 2'

expect '230' '
def main () {
	let reports = File.open("test/spec/examples/aoc2024/d1-input.txt")
						.unwrap()
						.read_to_str()
						.split("\n")
	let mut safe_reports = 0

	for report in reports {
		let levels = report.split(" ")
		if levels.len() < 3 {
			safe_reports += 1
			continue
		}

		let mut last_level = levels.at_raw(0).Int().unwrap()
		let mut is_increasing = levels.at_raw(1).Int().unwrap() > last_level

		for i in range(1, levels.len()) {
			let level = levels.at_raw(i).Int().unwrap()

			let diff = (level - last_level).abs()
			if diff == 0 || diff < 1 || diff > 3 ->
				break

			if (is_increasing && level < last_level)
				|| (!is_increasing && level > last_level)
			{
				break
			}

			last_level = level

			if i == levels.len() - 1 {
				safe_reports += 1
			}
		}
	}

	print(safe_reports.Str())
}
'