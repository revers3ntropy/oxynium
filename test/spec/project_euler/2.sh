describe 'Project Euler #2: Even Fibonacci numbers'

perf_test_comp_cpp 50 '4613732' '
    fn main () {
        let mut sum = 0;
        let mut a = 1;
        let mut b = 2;
        while {
            if b % 2 == 0 {
                sum = sum + b;
            };
            let c = a + b;
            a = b;
            b = c;
            if b >= 4000000 {
                break;
            };
        };
        print(sum.str());
    }
' '
    #include <iostream>

    int main () {
        int sum = 0;
        int a = 1;
        int b = 2;
        while (true) {
            if (b % 2 == 0) {
                sum = sum + b;
            }
            int c = a + b;
            a = b;
            b = c;
            if (b >= 4000000) {
                break;
            }
        }
        std::cout << sum;
    }
'