describe 'Project Euler #7: 10001st prime'

perf_test_comp_cpp 1 '104743' '
    def main () {
        let mut i = 1
        let mut j = 0
        while {
            i += 1
            let mut is_prime = true
            for k in range(2, i) {
                if i % k == 0 {
                    is_prime = false
                    break
                }
            }
            if is_prime {
                j += 1
                if j == 10001 {
                    print(i.Str())
                    break
                }
            }
        }
    }
' '
    #include <iostream>
    #include <cmath>

    int main () {
        int i = 1;
        int j = 0;
        while (true) {
            i += 1;
            bool is_prime = true;
            int k = 2;
            while (k < i) {
                if (i % k == 0) {
                    is_prime = false;
                    break;
                }
                k += 1;
            }
            if (is_prime) {
                j += 1;
                if (j == 10001) {
                    std::cout << i;
                    break;
                }
            }
        }
    }
'

