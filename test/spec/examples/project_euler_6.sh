describe 'Project Euler #6: Sum square difference'

perf_test_comp_cpp 50 '25164150' '
    def main () {
        let mut sum_of_squares = 0
        let mut square_of_sum = 0
        for i in range(1, 101) {
            sum_of_squares = sum_of_squares + i * i
            square_of_sum = square_of_sum + i
        }
        square_of_sum = square_of_sum * square_of_sum
        print((square_of_sum - sum_of_squares).Str())
    }
' '
    #include <iostream>

    int main () {
        int sum_of_squares = 0;
        int square_of_sum = 0;
        int i = 1;
        while (i <= 100) {
            sum_of_squares = sum_of_squares + i * i;
            square_of_sum = square_of_sum + i;
            i += 1;
        }
        square_of_sum = square_of_sum * square_of_sum;
        std::cout << square_of_sum - sum_of_squares;
    }
'