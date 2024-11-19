describe 'Project Euler #3: Largest prime factor'

perf_test_comp_cpp 50 '6857' '
    def main () {
        let mut n = 600851475143
        let mut i = 2
        let mut max = 0
        while {
            if i * i > n ->
                break
            if n % i == 0 {
                n /= i;
                max = i;
            } else {
                i += 1
            }
        }
        if n > max {
            max = n
        }
        print(max.Str())
    }
' '
    #include <iostream>

    int main () {
        long long n = 600851475143;
        long long i = 2;
        long long max = 0;
        while (true) {
            if (i * i > n) {
                break;
            }
            if (n % i == 0) {
                n /= i;
                max = i;
            } else {
                i += 1;
            }
        }
        if (n > max) {
            max = n;
        }
        std::cout << max;
        return 0;
    }
'