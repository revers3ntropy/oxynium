describe 'Static Methods on Classes'

expect 'Hello World!' '
    class C {
        fn a() Str {
            return "Hello World!"
        }
    }
    print(C.a())
'
expect 'Hello World!' '
    class C {
        fn a(msg: Str) {
            print(msg)
        }
    }
    C.a("Hello World!")
'
expect 'Hello World!' '
    class C {
        msg: Str,

        fn make(msg: Str) C {
            return new C {
                msg: msg
            }
        }

        fn log(self) {
            print(self.msg)
        }
    }
    C.make("Hello World!").log()
'
expect_err 'TypeError' '
    class C {
        fn log(msg: Str) {
            print(msg)
        }
    }
    (new C).log("Hello World!")
'
expect 'Hello World!' '
    class C {
        fn log(c: C, msg: Str) {
            print(msg)
        }
    }
    (new C).log("Hello World!")
'