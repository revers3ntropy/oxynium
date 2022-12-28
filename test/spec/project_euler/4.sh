describe 'Project Euler #4: Largest palindrome product'

perf_test_comp_cpp 1 '906609' '
    fn is_palindrome(_n: Int) Bool {
        let mut digits = "";
        let mut n = _n;
        while n > 0 {
            digits = digits + (n % 10).str();
            n = n / 10
        }
        let mut i = 0;
        let mut j = digits.len() - 1;
        while i < j {
            if digits.at(i) != digits.at(j) {
                return false
            }
            i = i + 1;
            j = j - 1;
        }
        return true
    }

    fn main() {
        let mut max = 0;
        let mut i = 100;
        while i < 1000 {
            let mut j = 100;
            while j < 1000 {
                let prod = i * j;
                if prod > max && is_palindrome(prod) {
                    max = prod;
                }
                j = j + 1;
            }
            i = i + 1;
        }
        print(max.str());
    }
' '
    #include <stdio.h>
    #include <stdbool.h>
    #include <string.h>

    bool is_palindrome(int n) {
        char digits[6];
        int i = 0;
        while (n > 0) {
            digits[i] = n % 10 + 48;
            n = n / 10;
            i = i + 1;
        }
        digits[i] = 0;
        int j = 0;
        int k = strlen(digits) - 1;
        while (j < k) {
            if (digits[j] != digits[k]) {
                return false;
            }
            j = j + 1;
            k = k - 1;
        }
        return true;
    }

    int main() {
        int max = 0;
        int i = 100;
        while (i < 1000) {
            int j = 100;
            while (j < 1000) {
                int prod = i * j;
                if (prod > max && is_palindrome(prod)) {
                    max = prod;
                }
                j = j + 1;
            }
            i = i + 1;
        }
        printf("%d", max);
        return 0;
    }
'