describe 'Class Instance Field Access'

expect '1' '
    class S { x: Int };
    print(new S { x: 1 }.x.Str());
'
expect '456' '
    class S { x: Int, };
    class S2 { s: S };
    // different bracket permutations
    print(new S2 { s: new S { x: 4 }}.s.x.Str());
    print((new S2 { s: new S { x: 5 }}).s.x.Str());
    print((new S2 { s: new S { x: 6 }}.s).x.Str());
'
expect '9hi' '
    class S {
        x: Int,
        y: Str
    };
    def f () {
        let s = new S { x: 9, y: "hi" };
        print(s.x.Str());
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
    def main () {
        let s = new S {
            x: 2,
            y: 3
        };
        print(s.x.Str());
        print(s.y.Str());
    }
'
expect '3223' '
    class S {
        b: Int,
        a: Int
    }
    def main () {
        let a = new S {
            a: 2,
            b: 3,
        };
        print(a.b.Str());
        print(a.a.Str());

        let b = new S {
            b: 3,
            a: 2,
        };
        print(b.a.Str());
        print(b.b.Str());
    }
'