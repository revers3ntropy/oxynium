describe 'Project Euler #3: Largest prime factor'

perf_test_comp_cpp 150 '6857' '
    var n = 600851475143;
    var i = 2;
    var max = 0;
    for {
        if i * i > n {
            break;
        };
        if n % i == 0 {
            n = n / i;
            max = i;
        } else {
            i = i + 1;
        };
    };
    if n > max {
        max = n;
    };
    print(max.str());
' '
    #include <iostream>

    int main () {
        long n = 600851475143;
        int i = 2;
        int max = 0;
        while (true) {
            if (i * i > n) {
                break;
            }
            if (n % i == 0) {
                n = n / i;
                max = i;
            } else {
                i = i + 1;
            }
        }
        if (n > max) {
            max = n;
        }
        std::cout << max;
    }
'