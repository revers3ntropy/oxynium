describe 'Project Euler #1: Multiples of 3 or 5'

perf_test_comp_cpp 100 '233168' '
    var sum = 0;
    var i = 0;
    for {
        if i % 3 == 0 || i % 5 == 0 {
            sum = sum + i;
        };
        i = i + 1;
        if i >= 1000 {
            break;
        };
    };
    print_int(sum);
' '
    #include <iostream>

    int main () {
        int sum = 0;
        int i = 0;
        while (true) {
            if (i % 3 == 0 || i % 5 == 0) {
                sum = sum + i;
            }
            i = i + 1;
            if (i >= 1000) {
                break;
            }
        }
        std::cout << sum;
    }
'