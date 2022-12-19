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
expect '' '
    fn main () {
        class C;
    }
'
expect_err 'UnknownSymbol' '
    fn main () {
        class C
    }
    fn f(a: C);
'
expect_err 'UnknownSymbol' '
    fn f(a: C);
    fn main () {
        class C
    }
'
expect_err 'SyntaxError' '
    class A {
        class B {}
    }
'


describe 'Class Instantiation'

expect '' '
    class S { x: Int }
    new S { x: 1 };
'
expect '' '
    class S {
        x: Int,
        y: Bool,
    };
    new S { x: 1, y: true };
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
expect_err 'UnknownSymbol' 'new s'
expect_err 'SyntaxError' 'new 1'
expect_err 'SyntaxError' 'new ""'
expect_err 'SyntaxError' 'new new C'
expect_err 'SyntaxError' 'new C()'


describe 'Primitives'

expect '' '
    primitive Q {}
    primitive P
'
expect '' '
    primitive P {
        fn f(self) {}
    }
'
expect '' '
    primitive P {
        extern fn f(self): Int,
        fn g(self) {}
    }
'
expect 'hi hi ' '
    primitive P {
        fn log(self) {
            print("hi ");
        }
    }
    new P{}.log();
    new P.log();
'
expect_err 'SyntaxError' '
    primitive P {
        x: Int
    }
'
expect_err 'SyntaxError' '
    primitive P {
        fn f(self) {}
        x: Int
    }
'


describe 'Composition'

expect '13' '
    class S {
        x: Int,
        y: Int
    }
    class S2 {
        s: S,
        z: Int
    }
    fn main () {
        let s2 = new S2 {
            s: new S {
                x: 1, y: 2
            },
            z: 3
        };
        print(s2.s.x.str());
        print(s2.z.str());
    }
'

expect '12' '
    class S2 {
        x: Int
    }
    class S {
        x: Int,
        fn make_s2(self): S2 {
            return new S2 {
                x: self.x
            }
        }
    }
    fn main () {
        print(new S { x: 1 }.make_s2().x.str());
        let s = new S { x: 2 };
        print(s.make_s2().x.str());
    }
'


describe 'Static Functions'

expect 'hi' '
    class S {
        msg: Str,
        fn f(self): Str {
            return self.msg;
        }
    }
    fn main () {
        let s = new S { msg: "hi" };
        print(S.f(s));
    }
'
expect 'abc' '
    class S {
        fn f(self, msg: Str): Str {
            return msg;
        }
    }
    fn main () {
        print(S.f(new S, "abc"));
    }
'
expect 'hello' '
    class S {
        fn f(self, a: Int, msg: Str = "hello"): Str {
            return msg;
        }
    }
    fn main () {
        print(S.f(new S, 1));
    }
'