describe 'Recursion'

expect '55' '
    fn fib(n: Int) Int {
        if n <= 1 {
            return n;
        };
        return fib(n - 1) + fib(n - 2);
    };
    print(fib(10).str());
'
expect '0123456789' '
    fn a(i = 0) {
        if i > 9 {
            return
        }
        print(i.str());
        a(i + 1);
    }
    a()
'