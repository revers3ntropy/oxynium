describe 'Primitive Declaration'

expect '' '
    primitive Q {}
    primitive P
'
expect '' '
    primitive P {
        def f(self) {}
    }
'
expect '' '
    primitive P {
        extern def f(self) Int,
        def g(self) {}
    }
'
expect 'hi hi ' '
    primitive P {
        def log(self) {
            print("hi ");
        }
    }
    new P{}.log();
    new P.log();
'
expect_err 'SyntaxError' '
    primitive P {
        x: Int
    }
'
expect_err 'SyntaxError' '
    primitive P {
        def f(self) {}
        x: Int
    }
'
