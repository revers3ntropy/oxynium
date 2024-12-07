describe 'AOC 2024 Day 1'

expect '1223326' '
    def main () {
        let input_lines = File.open("test/spec/examples/aoc2024/d1-input.txt")
                              .unwrap()
                              .read_to_str()
                              .split("\n")

        let mut left = List.with_capacity!<Int>(input_lines.length * 8)
        let mut right = List.with_capacity!<Int>(input_lines.length * 8)

        for line in input_lines {
            // assumes all lines are well-formed
            let parts = line.split("   ")
            left.push(parts.at_raw(0).Int().unwrap())
            right.push(parts.at_raw(1).Int().unwrap())
        }

        // as we cannot yet use methods as first-class functions,
        // use a work-around
        let int_compare = fn (a: Int, b: Int) Int -> Int.compare(a, b)

        left = left.sort(int_compare)
        right = right.sort(int_compare)

        let mut diff = 0
        for i in range(0, left.len()) {
            diff += (left.at_raw(i) - right.at_raw(i)).abs()
        }
        print(diff.Str())
    }
'