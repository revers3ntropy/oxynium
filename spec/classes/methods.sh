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
