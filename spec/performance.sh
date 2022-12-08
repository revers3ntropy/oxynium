describe 'Performance'

oxy_perf_code='
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
'

expect '49999995000000' "$oxy_perf_code"

perf_timer_start=$(date +"%s.%3N")

errors=$({
    # in.oxy already has code in
  ./target/release/oxynium -o=test-out --std std.asm in.oxy
} 2>&1 > /dev/null)

if [ "$errors" != "" ]; then
    test_failed 'Oxy perf test' 'no errors' "$errors"
fi

echo "Oxy Compilation: $(echo "$(date +"%s.%3N") - $perf_timer_start" | bc -l)"
perf_timer_start=$(date +"%s.%3N")

./test-out > /dev/null

echo "Oxy Execution: $(echo "$(date +"%s.%3N") - $perf_timer_start" | bc -l)"
perf_timer_start=$(date +"%s.%3N")

echo '
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
' > out.cpp

gcc out.cpp -lstdc++ -O3

echo "C++ Compilation: $(echo "$(date +"%s.%3N") - $perf_timer_start" | bc -l)"
perf_timer_start=$(date +"%s.%3N")

cpp_out=$(./a.out)

echo "C++ Execution: $(echo "$(date +"%s.%3N") - $perf_timer_start" | bc -l)"
perf_timer_start=$(date +"%s.%3N")

if [ "$cpp_out" != "49999995000000" ]; then
    test_failed "C++ performance code" "49999995000000" "$cpp_out"
fi