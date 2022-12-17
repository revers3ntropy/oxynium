describe 'Class Declarations'

expect '' 'class S {};'
expect '' '
    class S {};
    fn do_nothing(s: S) {};
'
expect '' '
    class S {
        x: Int,
        y: Int
    };
'
expect_err '' '
    class S { s: S };
'


describe 'Class Instantiation'

expect '' '
    class S {
        x: Int,
        y: Int,
    };
    new S { x: 1, y: 2 };
'
expect_err 'TypeError' '
    class S { x: Int };
    class S2 { s: S };
    new S2 { s: new S { x: "hi" } };
'
expect_err 'TypeError' '
    class S { x: Int };
    new S {};
'
expect_err 'TypeError' '
    class S { x: Int };
    new S { y: 1 };
'
expect_err 'TypeError' '
    class S { x: Int };
    new S { x: "hi" };
'
expect_err 'TypeError' '
    class S { x: Int };
    new S { x: 1, y: 2 };
'


describe 'Class Field Access'

expect '' '
    class S { x: Int };
    (new S { x: 1 }).x;
'
expect '1' '
    class S { x: Int };
    print_int(new S { x: 1 }.x);
'
expect '456' '
    class S { x: Int, };
    class S2 { s: S };
    // different bracket permutations
    print_int(new S2 { s: new S { x: 4 }}.s.x);
    print_int((new S2 { s: new S { x: 5 }}).s.x);
    print_int((new S2 { s: new S { x: 6 }}.s).x);
'
expect '9hi' '
    class S {
        x: Int,
        y: Str
    };
    fn f () {
        let s = new S { x: 9, y: "hi" };
        print_int(s.x);
        print(s.y);
    };
    f();
'
expect_err 'TypeError' '
    class S { x: Int };
    new S { x: 1 }.y;
'
expect_err 'TypeError' '
    class S { x: Int };
    print(new S { x: 1 }.x);
'