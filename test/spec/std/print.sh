describe 'func print'

expect 'Hello, World!' 'print("Hello, World!")'
expect_err 'TypeError' 'print(1)'
expect_err 'TypeError' 'print(true)'
expect_err 'TypeError' 'print("", "")'
expect_err 'TypeError' 'print("", true)'

expect '42135' '
    func g() {
        print(1.str());
    };
    func f() {
        print(2.str());
        g();
        print(3.str());
    };
    print(4.str());
    f();
    print(5.str());
'