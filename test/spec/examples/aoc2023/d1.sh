describe 'AOC Day 1'

expect '53921' '
    def main () {
        let input_lines = File.open("test/spec/examples/aoc2023/d1-input.txt")
                              .unwrap()
                              .read_to_str()
                              .split("\n")
        let mut sum = 0
        let mut row_idx = 0
        while (row_idx < input_lines.len()) {
            let line = input_lines.at(row_idx).unwrap()
            if line == "" ->
                break

            let mut num = ""
            let mut i = 0
            while (true) {
                if line.at(i).is_digit() {
                    num += line.at(i).Str()
                    break
                }
                i += 1
            }

            i = line.len() - 1
            while (true) {
                if line.at(i).is_digit() {
                    num += line.at(i).Str()
                    break
                }
                i -= 1
            }

            sum += num.Int().unwrap()
            row_idx += 1
        }

        print(sum.Str())
    }
'