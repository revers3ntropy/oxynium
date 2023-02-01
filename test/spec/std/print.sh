describe 'func print'

expect 'Hello, World!' 'print("Hello, World!")'
expect_err 'TypeError' 'print(1)'
expect_err 'TypeError' 'print(true)'
expect_err 'TypeError' 'print("", "")'
expect_err 'TypeError' 'print("", true)'

expect '42135' '
    func g() {
        print(1.Str());
    };
    func f() {
        print(2.Str());
        g();
        print(3.Str());
    };
    print(4.Str());
    f();
    print(5.Str());
'