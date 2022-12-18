describe 'fn Bool.sh'

expect 'true' 'print(true.str())'
expect 'false' 'print(false.str())'
expect 'true' '
    fn f() {
        let a = true;
        print(a.str());
    };
    f();
'
expect 'true' '
    fn f() {
        let a = true;
        let a_str = a.str();
        print(a_str);
    };
    f();
'
expect 'false' '
    fn f() {
        let a_str = false.str();
        print(a_str);
    };
    f();
'