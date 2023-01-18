describe 'Runtime Performance'

perf_test_comp_cpp 1 '149995000' '
    const n = 10000;
    fn main () {
        let mut sum = 0;
        let mut i = 0;
        while {
            if i >= n {
                break;
            };
            let mut j = 0;
            while {
                if j >= n {
                    break;
                };
                sum = sum + 1;
                j = j + 1;
            };
            sum = sum + i;
            i = i + 1;
        };
        print(sum.str());
    }
' '
    #include <iostream>

    int main () {
        long sum = 0;
        int n = 10000;
        int i = 0;
        while (true) {
            if (i >= n) {
                break;
            }
            int j = 0;
            while (true) {
                if (j >= n) {
                    break;
                }
                sum = sum + 1;
                j = j + 1;
            }
            sum = sum + i;
            i = i + 1;
        }
        std::cout << sum;
    }
'


describe 'Compiletime Performance'

perf_test_comp_cpp 1 '2001000' "
    fn main () {
        let mut i = 0;
        $(printf 'i = i + %d;\n' {1..2000})
        print(i.str());
    }
" "
    #include <iostream>

    int main () {
        int i = 0;
        $(printf 'i = i + %d;\n' {1..2000})
        std::cout << i;
    }
"