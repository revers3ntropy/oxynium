describe 'Static Methods on Classes'

expect 'Hello World!!' '
    class C {
        def a() Str {
            return "Hello World!"
        }
    }
    print(C.a())

    class D {
        def d() -> "!"
    }
    print(D.d())
'
expect 'Hello World!' '
    class C {
        def a(msg: Str) {
            print(msg)
        }
    }
    C.a("Hello World!")
'
expect 'Hello World!' '
    class C {
        msg: Str,

        def make(msg: Str) C {
            return new C {
                msg: msg
            }
        },

        def log(self) {
            print(self.msg)
        }
    }
    C.make("Hello World!").log()
'
expect_err 'TypeError' '
    class C {
        def log(msg: Str) {
            print(msg)
        }
    }
    (new C).log("Hello World!")
'
expect 'Hello World!' '
    class C {
        def log(c: C, msg: Str) {
            print(msg)
        }
    }
    (new C).log("Hello World!")
'

expect 'hi' '
    class S {
        msg: Str,
        def f(self) Str {
            return self.msg;
        }
    }
    def main () {
        let s = new S { msg: "hi" };
        print(S.f(s));
    }
'
expect 'abc' '
    class S {
        def f(self, msg: Str) Str {
            return msg;
        }
    }
    def main () {
        print(S.f(new S, "abc"));
    }
'
expect 'hello' '
    class S {
        def f(self, a: Int, msg: Str = "hello") Str {
            return msg;
        }
    }
    def main () {
        print(S.f(new S, 1));
    }
'
expect_err 'TypeError' '
    class S;
    new S.f();
'
expect_err 'TypeError' '
    class S {
        def f(self){}
    };
    new S.g();
'
