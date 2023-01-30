describe 'Project Euler #5: Smallest multiple'

perf_test_comp_cpp 1 '232792560' '
    func main () {
        let mut i = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
        while {
            i = i + 1;

            let mut j = 2;
            let mut is_divisible = true;
            while j < 20 {
                if i % j != 0 {
                    is_divisible = false;
                    break
                }
                j = j + 1;
            }
            if is_divisible {
                print(i.str());
                break
            }
        }
    }
' '
    #include <iostream>

    int main () {
        int i = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
        while (true) {
            i = i + 1;

            int j = 2;
            bool is_divisible = true;
            while (j < 20) {
                if (i % j != 0) {
                    is_divisible = false;
                    break;
                }
                j = j + 1;
            }
            if (is_divisible) {
                std::cout << i;
                break;
            }
        }
    }
'