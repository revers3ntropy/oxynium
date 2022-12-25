describe 'Char'

expect '' 'print(new Char.str())'
expect_err 'TypeError' 'print(new Char)'


describe 'fn Char.str'

# TODO: requires char literals


describe 'fn Char.=='

expect_expr_bool 'true'  '"a".at(0) == "a".at(0)'
expect_expr_bool 'false' '"a".at(0) == "b".at(0)'
expect_expr_bool 'true'  '"💖".at(0) == "💖".at(0)'
expect_expr_bool 'true'  '"🇨🇦".at(0) == "🇦".at(0)'


describe 'fn Char.!='

expect_expr_bool 'false' '"a".at(0) != "a".at(0)'
expect_expr_bool 'true'  '"a".at(0) != "b".at(0)'
expect_expr_bool 'false' '"💖".at(0) != "💖".at(0)'
expect_expr_bool 'false' '"🇨🇦".at(0) != "🇦".at(0)'