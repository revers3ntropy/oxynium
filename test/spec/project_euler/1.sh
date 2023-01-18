describe 'Project Euler #1: Multiples of 3 or 5'

perf_test_comp_cpp 50 '233168' '
    fn main () {
        let mut sum = 0;
        let mut i = 0;
        while i < 1000 {
            if i % 3 == 0 || i % 5 == 0 {
                sum = sum + i;
            };
            i = i + 1;
        };
        print(sum.str());
    }
' '
    #include <iostream>

    int main () {
        int sum = 0;
        int i = 0;
        while (i < 1000) {
            if (i % 3 == 0 || i % 5 == 0) {
                sum = sum + i;
            }
            i = i + 1;
        }
        oxy_std::cout << sum;
    }
'