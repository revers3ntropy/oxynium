describe 'Project Euler #2: Even Fibonacci numbers'

perf_test_comp_cpp 100 '4613732' '
    var sum = 0;
    var a = 1;
    var b = 2;
    var c = 0;
    for {
        if b % 2 == 0 {
            sum = sum + b;
        };
        c = a + b;
        a = b;
        b = c;
        if b >= 4000000 {
            break;
        };
    };
    print(sum.str());
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