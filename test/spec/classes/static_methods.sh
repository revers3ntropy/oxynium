describe 'Calling Methods Statically'

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