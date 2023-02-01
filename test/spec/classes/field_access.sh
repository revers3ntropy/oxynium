describe 'Class Instance Field Access'

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
    func f () {
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
expect '23' '
    class S {
        x: Int,
        y: Int
    }
    func main () {
        let s = new S {
            x: 2,
            y: 3
        };
        print(s.x.str());
        print(s.y.str());
    }
'
expect '3223' '
    class S {
        b: Int,
        a: Int
    }
    func main () {
        let a = new S {
            a: 2,
            b: 3,
        };
        print(a.b.str());
        print(a.a.str());

        let b = new S {
            b: 3,
            a: 2,
        };
        print(b.a.str());
        print(b.b.str());
    }
'