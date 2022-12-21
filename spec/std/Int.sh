describe 'Int'

expect '0' 'print(new Int.str())'


describe 'fn Int.str'

expect '1' 'print(1.str())'
expect '123' 'print(123.str())'
expect '1234567890' 'print(1234567890.str())'
expect '0' 'print(0.str())'
expect '-1' 'print((-1).str())'
expect '-123' 'print((-123).str())'
expect '-1234567890' 'print((-1234567890).str())'
expect '0' '
    const a = 0;
    print(a.str());
'
expect '-106709' '
    fn f() {
        let a = -106709;
        let a_str = a.str();
        print(a_str);
    };
    f();
'


describe 'fn Int.max'

expect_expr_int '2' '1.max(2)'
expect_expr_int '2' '2.max(1)'
expect_expr_int '2' '2.max(2)'
expect_expr_int '-1' '-1.max(0)'
expect_expr_int '0' '(-1).max(0)'
expect_expr_int '1' '(-1).max(1)'
expect_expr_int '-10' '(-100).max(-10)'
expect_expr_int '-10' '(-10).max(-100)'
expect_expr_int '0' '0.max(0)'
expect_expr_int '1' '0.max(1)'
expect_expr_int '1' '1.max(0)'
expect_expr_int '1' '1.max(1)'

describe 'fn Int.min'

expect_expr_int '1' '1.min(2)'
expect_expr_int '1' '2.min(1)'
expect_expr_int '2' '2.min(2)'
expect_expr_int '-1' '(-1).min(0)'
expect_expr_int '-1' '(-1).min(1)'
expect_expr_int '-100' '(-100).min(-10)'
expect_expr_int '0' '0.min(1)'
expect_expr_int '0' '1.min(0)'
expect_expr_int '0' '0.min(0)'
expect_expr_int '1' '1.min(1)'