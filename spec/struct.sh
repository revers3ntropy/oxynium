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

expect '' '
    struct S {
        x: Int;
        y: Int;
    };
    new S { x: 1, y: 2 };
'


describe 'Struct Field Access'

expect '1' '
    struct S {
        x: Int;
    };
    print_int(new S { x: 1 }.x);
'
expect '456' '
    struct S {
        x: Int;
    };
    struct S2 {
        s: S;
    };
    // different bracket permutations
    print_int(new S2 { s: new S { x: 4 }}.s.x);
    print_int((new S2 { s: new S { x: 5 }}).s.x);
    print_int((new S2 { s: new S { x: 6 }}.s).x);
'
expect '9hi' '
    struct S {
        x: Int;
        y: Str;
    };
    fn f () {
        let s = new S { x: 9, y: "hi" };
        print_int(s.x);
        print(s.y);
    };
    f();
'