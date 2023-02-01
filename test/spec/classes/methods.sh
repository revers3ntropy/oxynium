describe 'Methods on Classes'

expect 'x = 1, x = 2, ' '
    class S {
        x: Int,
        func log(self) Void {
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
        func log(self) {
           self.x;
        }
    }
'
expect '' '
    class S {
        x: Int,
        extern func f1(self) S,
        func f2(self) {}
        extern func f3(self),
        y: Int,
        func f4(self) {},
        extern func f5(a: Int,),
        z: Str
    }
'
expect_err 'TypeError' '
    class S {
        func f(self, a: Int) {}
    };
    (new S{}).f();
'
expect_err 'TypeError' '
    class S {
        func f(self, a: Int) {}
    }
    (new S).f("");
'
expect '' '
    class S {
        func f(self, a: Int) {}
    }
    (new S{}).f(1);
    new S.f(1);
'


describe 'Default Parameters on Methods'

expect 'hi' '
    class S {
        func f(self, msg: Str = "hi") {
            print(msg);
        }
    };
    new S{}.f();
'
expect 'hello world' '
    class A {
        func f(self, msg: Str = "hello") {
            print(msg);
        }
    };
    class B {
        func f(self, msg: Str = " world") {
            print(msg);
        }
    };
    new A.f();
    new B.f();
'


describe 'Non-Static Methods Require self Parameter'

expect_err 'SyntaxError' '
    class S {
        func log(self: S) {}
    }
'
expect_err 'SyntaxError' '
    class S {
        func log(self: Int) {}
    }
'