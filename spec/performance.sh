describe 'Performance'

perf_test_comp_cpp '49999995000000' '
    var sum = 0;
    const n = 10000000;
    var i = 0;
    for {
        if i >= n {
            break;
        };
        sum = sum + i;
        i = i + 1;
    };
    print_int(sum);
' '
    #include <iostream>

    int main () {
        long sum = 0;
        int n = 10000000;
        int i = 0;
        while (true) {
            if (i >= n) {
                break;
            }

            sum = sum + i;
            i = i + 1;
        }
        std::cout << sum;
    }
'