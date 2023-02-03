describe 'Recursion'

expect '55' '
    def fib(n: Int) Int {
        if n <= 1 {
            return n;
        };
        return fib(n - 1) + fib(n - 2);
    };
    print(fib(10).Str());
'
expect '0123456789' '
    def a(i = 0) {
        if i > 9 {
            return
        }
        print(i.Str());
        a(i + 1);
    }
    a()
'