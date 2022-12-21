describe 'Str'

expect '' 'print(new Str.str().str())'
expect '' 'print(new Str.str())'
expect '' 'print(new Str)'


describe 'fn Str.str'

expect '' 'print("".str())'
expect 'hi' 'print("hi".str())'


describe 'fn Str.len'

expect_expr_int '0' '"".len()'
expect_expr_int '2' '"hi".len()'
expect_expr_int '3' '"abc".len()'
expect_expr_int '4' '"abcd".len()'
expect_err 'TypeError' 'print("abcd".len())'


describe 'fn Str.at'

expect '' 'print("abc".at(-1).str())'
expect 'a' 'print("abc".at(0).str())'
expect 'b' 'print("abc".at(1).str())'
expect 'c' 'print("abc".at(2).str())'
expect '' 'print("abc".at(4).str())'
expect '💖' 'print("💖💖".at(0).str())'