describe 'Class Instance Field Access'

expect '' '
    class S { x: Int };
    (new S { x: 1 }).x;
'
expect '1' '
    class S { x: Int };
    print(new S { x: 1 }.x.str());
'
expect '456' '
    class S { x: Int, };
    class S2 { s: S };
    // different bracket permutations
    print(new S2 { s: new S { x: 4 }}.s.x.str());
    print((new S2 { s: new S { x: 5 }}).s.x.str());
    print((new S2 { s: new S { x: 6 }}.s).x.str());
'
expect '9hi' '
    class S {
        x: Int,
        y: Str
    };
    fn f () {
        let s = new S { x: 9, y: "hi" };
        print(s.x.str());
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