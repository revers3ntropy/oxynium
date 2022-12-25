describe 'Calling Methods Statically'

expect 'hi' '
    class S {
        msg: Str,
        fn f(self) Str {
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
        fn f(self, msg: Str) Str {
            return msg;
        }
    }
    fn main () {
        print(S.f(new S, "abc"));
    }
'
expect 'hello' '
    class S {
        fn f(self, a: Int, msg: Str = "hello") Str {
            return msg;
        }
    }
    fn main () {
        print(S.f(new S, 1));
    }
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