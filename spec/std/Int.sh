describe 'fn Int.str'

expect '1' 'print(1.str())'
expect '123' 'print(123.str())'
expect '1234567890' 'print(1234567890.str())'
expect '0' 'print(0.str())'
expect '-1' 'print((-1).str())'
expect '-123' 'print((-123).str())'
expect '-1234567890' 'print((-1234567890).str())'
expect '0' '
    const a = 0;
    print(a.str());
'
expect '-106709' '
    fn f() {
        let a = -106709;
        let a_str = a.str();
        print(a_str);
    };
    f();
'