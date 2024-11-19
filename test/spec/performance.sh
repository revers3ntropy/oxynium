describe 'Runtime Performance'

perf_test_comp_cpp 1 '149995000' '
    const n = 10000
    def main () {
        let mut sum = 0
        for i in range(n) {
            for j in range(n) {
                sum = sum + 1
            }
            sum = sum + i
        }
        print(sum.Str())
    }
' '
    #include <iostream>

    int main () {
        long sum = 0;
        int n = 10000;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                sum = sum + 1;
            }
            sum = sum + i;
        }
        std::cout << sum;
    }
'


describe 'Compiletime Performance'

perf_test_comp_cpp 1 '2001000' "
    def main () {
        let mut i = 0;
        $(printf 'i = i + %d;\n' {1..2000})
        print(i.Str());
    }
" "
    #include <iostream>

    int main () {
        int i = 0;
        $(printf 'i = i + %d;\n' {1..2000})
        std::cout << i;
    }
"