describe 'Project Euler #1: Multiples of 3 or 5'

perf_test_comp_cpp 50 '233168' '
    def main () {
        let mut sum = 0
        for i in range(1000) {
            if i % 3 == 0 || i % 5 == 0 {
                sum += i
            }
        }
        print(sum.Str())
    }
' '
    #include <iostream>

    int main () {
        int sum = 0;
        int i = 0;
        while (i < 1000) {
            if (i % 3 == 0 || i % 5 == 0) {
                sum += i;
            }
            i += 1;
        }
        std::cout << sum;
    }
'