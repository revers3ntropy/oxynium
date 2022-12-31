describe 'Project Euler #7: 10001st prime'

perf_test_comp_cpp 1 '104743' '
    fn main () {
        let mut i = 1;
        let mut j = 0;
        while {
            i = i + 1;
            let mut is_prime = true;
            let mut k = 2;
            while k < i {
                if i % k == 0 {
                    is_prime = false;
                    break
                }
                k = k + 1;
            }
            if is_prime {
                j = j + 1;
                if j == 10001 {
                    print(i.str());
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
            i = i + 1;
            bool is_prime = true;
            int k = 2;
            while (k < i) {
                if (i % k == 0) {
                    is_prime = false;
                    break;
                }
                k = k + 1;
            }
            if (is_prime) {
                j = j + 1;
                if (j == 10001) {
                    std::cout << i;
                    break;
                }
            }
        }
    }
'

