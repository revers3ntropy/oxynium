describe 'AOC Day 1'

expect '53921' '
    def main () {
        let input_lines = File.open("test/spec/examples/aoc2023/d1-input.txt")
                              .unwrap()
                              .read_to_str()
                              .split("\n")
        let mut sum = 0
        for line in input_lines {
            if line == "" ->
                break

            let mut num = ""
            for c in line {
                if c.is_digit() {
                    num += c.Str()
                    break
                }
            }

            let mut i = line.len() - 1
            while (true) {
                if line.at(i).is_digit() {
                    num += line.at(i).Str()
                    break
                }
                i -= 1
            }

            sum += num.Int().unwrap()
        }

        print(sum.Str())
    }
'