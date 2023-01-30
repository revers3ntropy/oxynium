describe 'Static Methods on Classes'

expect 'Hello World!' '
    class C {
        func a() Str {
            return "Hello World!"
        }
    }
    print(C.a())
'
expect 'Hello World!' '
    class C {
        func a(msg: Str) {
            print(msg)
        }
    }
    C.a("Hello World!")
'
expect 'Hello World!' '
    class C {
        msg: Str,

        func make(msg: Str) C {
            return new C {
                msg: msg
            }
        }

        func log(self) {
            print(self.msg)
        }
    }
    C.make("Hello World!").log()
'
expect_err 'TypeError' '
    class C {
        func log(msg: Str) {
            print(msg)
        }
    }
    (new C).log("Hello World!")
'
expect 'Hello World!' '
    class C {
        func log(c: C, msg: Str) {
            print(msg)
        }
    }
    (new C).log("Hello World!")
'