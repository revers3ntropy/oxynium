describe 'Primitives'

expect '' '
    primitive Q {}
    primitive P
'
expect '' '
    primitive P {
        fn f(self) {}
    }
'
expect '' '
    primitive P {
        extern fn f(self): Int,
        fn g(self) {}
    }
'
expect 'hi hi ' '
    primitive P {
        fn log(self) {
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
        fn f(self) {}
        x: Int
    }
'
