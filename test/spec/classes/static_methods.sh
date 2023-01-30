describe 'Calling Methods Statically'

expect 'hi' '
    class S {
        msg: Str,
        func f(self) Str {
            return self.msg;
        }
    }
    func main () {
        let s = new S { msg: "hi" };
        print(S.f(s));
    }
'
expect 'abc' '
    class S {
        func f(self, msg: Str) Str {
            return msg;
        }
    }
    func main () {
        print(S.f(new S, "abc"));
    }
'
expect 'hello' '
    class S {
        func f(self, a: Int, msg: Str = "hello") Str {
            return msg;
        }
    }
    func main () {
        print(S.f(new S, 1));
    }
'
expect_err 'TypeError' '
    class S;
    new S.f();
'
expect_err 'TypeError' '
    class S {
        func f(self){}
    };
    new S.g();
'