describe 'Project Euler #4: Largest palindrome product'

perf_test_comp_cpp 1 '906609' '
    fn main() {
        let mut max = 0;
        let mut i = 100;
        while i < 1000 {
            let mut j = 100;
            while j < 1000 {
                let prod = i * j;
                if prod > max {
                    let prod_str = prod.str();
                    if prod_str == prod_str.reversed()  {
                        max = prod;
                    }
                }
                j = j + 1;
            }
            i = i + 1;
        }
        print(max.str());
    }
' '
    #include <stdio.h>
    #include <string>
    #include <algorithm>

    int main() {
        int max = 0;
        int i = 100;
        while (i < 1000) {
            int j = 100;
            while (j < 1000) {
                int prod = i * j;
                if (prod > max) {
                    std::string str_prod = std::to_string(prod);
                    if (std::equal(str_prod.begin(), str_prod.end(),
                                   str_prod.rbegin())) {
                        max = prod;
                    }
                }
                j = j + 1;
            }
            i = i + 1;
        }
        printf("%d", max);
        return 0;
    }
'