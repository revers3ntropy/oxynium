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
expect '' '
    class S { s: S };
'
expect '' '
    class S;
'
expect_err 'TypeError' '
    class S {};
    class S {};
'
expect_err 'TypeError' '
    class Bool {};
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


describe 'Methods'

expect 'x = 1, x = 2, ' '
    class S {
        x: Int,
        fn log(self): Void {
           print("x = ");
           print(self.x.str());
           print(", ");
        }
    };
    new S { x: 1 }.log();
    new S { x: 2 }.log();
'
expect_err 'TypeError' '
    class S {
        fn log(self) {
           self.x;
        }
    }
'
expect_err 'SyntaxError' '
    class S {
        fn log(self: S) {}
    }
'
expect_err 'SyntaxError' '
    class S {
        fn log(self: Int) {}
    }
'
expect_err 'SyntaxError' '
    class S {
        fn log() {}
    }
'
expect '' '
    class S {
        x: Int,
        extern fn f1(self),
        fn f2(self) {}
        extern fn f3(self),
        y: Int,
        fn f4(self) {}
        z: Str
    }
'
expect_err 'TypeError' '
    class S {
        fn f(self, a: Int) {}
    };
    new S{}.f();
'
expect_err 'TypeError' '
    class S {
        fn f(self, a: Int) {}
    };
    new S{}.f("");
'
expect '' '
    class S {
        fn f(self, a: Int) {}
    };
    new S{}.f(1);
'
expect_err 'TypeError' '
    class S;
    new S.f();
'
expect_err 'TypeError' '
    class S {
        fn f(self){}
    };
    new S.g();
'
expect 'hi' '
    class S {
        fn f(self, msg: Str = "hi") {
            print(msg);
        }
    };
    new S{}.f();
'
expect 'hello world' '
    class A {
        fn f(self, msg: Str = "hello") {
            print(msg);
        }
    };
    class B {
        fn f(self, msg: Str = " world") {
            print(msg);
        }
    };
    new A.f();
    new B.f();
'