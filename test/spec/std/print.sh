describe 'def print'

expect 'Hello, World!' 'print("Hello, World!")'
expect_err 'TypeError' 'print(1)'
expect_err 'TypeError' 'print(true)'
expect '' 'print("", "")'
expect_err 'TypeError' 'print("", true)'

expect '42135' '
    def g() {
        print(1.Str());
    };
    def f() {
        print(2.Str());
        g();
        print(3.Str());
    };
    print(4.Str());
    f();
    print(5.Str());
'