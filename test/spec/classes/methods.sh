describe 'Methods on Classes'

expect 'x = 1, x = 2, ' '
    class S {
        x: Int,
        fn log(self) Void {
           print("x = ");
           print(self.x.str());
           print(", ");
        }
    }
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
expect '' '
    class S {
        x: Int,
        extern fn f1(self) S,
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
    (new S{}).f();
'
expect_err 'TypeError' '
    class S {
        fn f(self, a: Int) {}
    }
    (new S).f("");
'
expect '' '
    class S {
        fn f(self, a: Int) {}
    }
    (new S{}).f(1);
'
expect '' '
    class S {
        fn f(self, a: Int) {}
    }
    new S.f(1);
'


describe 'Default Parameters on Methods'

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


describe 'Non-Static Methods Require self Parameter'

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