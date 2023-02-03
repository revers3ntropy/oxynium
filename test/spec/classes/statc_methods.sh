describe 'Static Methods on Classes'

expect 'Hello World!' '
    class C {
        def a() Str {
            return "Hello World!"
        }
    }
    print(C.a())
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
        }

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