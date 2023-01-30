describe 'Primitive Declaration'

expect '' '
    primitive Q {}
    primitive P
'
expect '' '
    primitive P {
        func f(self) {}
    }
'
expect '' '
    primitive P {
        extern func f(self) Int,
        func g(self) {}
    }
'
expect 'hi hi ' '
    primitive P {
        func log(self) {
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
        func f(self) {}
        x: Int
    }
'
