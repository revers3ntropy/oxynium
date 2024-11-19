describe 'Project Euler #5: Smallest multiple'

perf_test_comp_cpp 1 '232792560' '
    def main () {
        let mut i = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
        while {
            i += 1;

            let mut is_divisible = true;
            for j in range(2, 20) {
                if i % j != 0 {
                    is_divisible = false;
                    break
                }
            }
            if is_divisible {
                print(i.Str());
                break
            }
        }
    }
' '
    #include <iostream>

    int main () {
        int i = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
        while (true) {
            i += 1;

            int j = 2;
            bool is_divisible = true;
            while (j < 20) {
                if (i % j != 0) {
                    is_divisible = false;
                    break;
                }
                j += 1;
            }
            if (is_divisible) {
                std::cout << i;
                break;
            }
        }
    }
'