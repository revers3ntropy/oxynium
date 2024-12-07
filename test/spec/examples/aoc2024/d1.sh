describe 'AOC 2024 Day 1'

expect '1223326' '
    def main () {
        let input_lines = File.open("test/spec/examples/aoc2024/d1-input.txt")
                              .unwrap()
                              .read_to_str()
                              .split("\n")

        let mut left = List.empty!<Int>()
        let mut right = List.empty!<Int>()

        for line in input_lines {
            let parts = line.split("   ")
            left.push(parts.at(0).unwrap().Int().unwrap())
            right.push(parts.at(1).unwrap().Int().unwrap())
        }

        let int_compare = fn (a: Int, b: Int) Int {
            if a < b -> return -1
            if a > b -> return 1
            return 0
        }
        left = left.sort(int_compare)
        right = right.sort(int_compare)

        let mut diff = 0
        for i in range(0, left.len()) {
            diff += (left.at_raw(i) - right.at_raw(i)).abs()
        }
        print(diff.Str())
    }
'