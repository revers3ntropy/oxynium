describe 'Struct Declarations'

expect '' 'struct S {};'
expect '' '
    struct S {};
    fn do_nothing(s: S) {};
'
expect '' '
    struct S {
        x: Int;
        y: Int;
    };
'


describe 'Struct Instantiation'

expect '12' '
    struct S {
        x: Int;
        y: Int;
    };
    fn f() {
        let s = S { x: 1, y: 2 };
        print_int(s.x);
        print_int(s.y);
    };
    f();
'