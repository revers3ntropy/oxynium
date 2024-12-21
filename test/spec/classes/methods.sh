describe 'Methods on Classes'

expect 'x = 1, x = 2, ' '
    class S {
        x: Int,
        def log(self) Void {
           print("x = ");
           print(self.x.Str());
           print(", ");
        }
    }
    new S { x: 1 }.log();
    new S { x: 2 }.log();
'
expect_err 'TypeError' '
    class S {
        def log(self) {
           self.x;
        }
    }
'
expect '' '
    class S {
        x: Int,
        extern def f1(self) S,
        def f2(self) {},
        extern def f3(self),
        y: Int,
        def f4(self) {},
        extern def f5(a: Int,),
        z: Str
    }
'
expect_err 'TypeError' '
    class S {
        def f(self, a: Int) {}
    };
    (new S{}).f();
'
expect_err 'TypeError' '
    class S {
        def f(self, a: Int) {}
    }
    (new S).f("");
'
expect '' '
    class S {
        def f(self, a: Int) {}
    }
    class T {
        def f(self, a: Int) -> a
    }

    def main () {
        (new S{}).f(1);
        new S.f(1);

        new T{}.f(1);
        (new T).f(1);
        new T.f(1);
    }
'


describe 'Default Parameters on Methods'

expect 'hi' '
    class S {
        def f(self, msg: Str = "hi") {
            print(msg);
        }
    };
    new S{}.f();
'
expect 'hello world' '
    class A {
        def f(self, msg: Str = "hello") {
            print(msg);
        }
    };
    class B {
        def f(self, msg: Str = " world") {
            print(msg);
        }
    };
    new A.f();
    new B.f();
'


describe 'self parameter cannot have type annotation'

expect_err 'SyntaxError' '
    class S {
        def log(self: S) {}
    }
'
expect_err 'SyntaxError' '
    class S {
        def log(self: Int) {}
    }
'


describe 'Can use static methods as first-class functions'

expect '1' '
class S {
    def g() -> 1
}
def main () {
    let g = S.g
    print(g().Str())
}
'
expect_err 'TypeError' '
class S {
    def g(self) -> 1
}
def main () {
    let g = S.g
    print(g().Str())
}
'
expect_err 'TypeError' '
class S {
    def g(self) -> 1
}
def main () {
    let g = (new S).g
    print(g().Str())
}
'
