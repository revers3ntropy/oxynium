describe 'primitive Bool'

expect 'false' 'print(new Bool.str())'
expect 'false' 'print(false.str())'
expect 'true' 'print(true.str())'

expect_expr_bool 'true' 'true'
expect_expr_bool 'false' 'false'

describe 'func Bool.str'

expect 'truetruefalse' '
    func main() {
        let a = true;
        print(a.str());

        let b = true;
        let b_str = b.str();
        print(b_str);

        let c_str = false.str();
        print(c_str);
    }
'


describe 'func Bool.&&'

expect_expr_bool 'true' 'true && true'
expect_expr_bool 'false' 'true && false'
expect_expr_bool 'false' 'false && true'
expect_expr_bool 'false' 'false && false'
expect_expr_bool 'true' 'true && true && true'
expect_expr_bool 'false' 'true && true && false'
expect_expr_bool 'false' 'true && false && true'
expect_expr_bool 'false' 'true && false && false'
expect_expr_bool 'false' 'false && true && true'
expect_expr_bool 'false' 'false && true && false'
expect_expr_bool 'false' 'false && false && true'
expect_expr_bool 'false' 'false && false && false'
expect_expr_bool 'true' 'true && true && true && true'
expect_err 'TypeError' '1 && true'
expect_err 'TypeError' 'true && ""'
expect_err 'TypeError' 'false && false && ""'


describe 'func Bool.||'

expect_expr_bool 'true' 'true || true'
expect_expr_bool 'true' 'true || false'
expect_expr_bool 'true' 'false || true'
expect_expr_bool 'false' 'false || false'
expect_expr_bool 'true' 'true || true || true'
expect_expr_bool 'true' 'true || true || false'
expect_expr_bool 'true' 'true || false || true'
expect_expr_bool 'true' 'true || false || false'
expect_expr_bool 'true' 'false || true || true'
expect_expr_bool 'true' 'false || true || false'
expect_expr_bool 'true' 'false || false || true'
expect_expr_bool 'false' 'false || false || false'
expect_expr_bool 'true' 'true || true || true || true'
expect_err 'TypeError' 'false || false || ""'
expect_err 'TypeError' 'false || 1'
expect_err 'TypeError' '1 || false'
expect_err 'TypeError' '1 || Str'
expect_err 'TypeError' 'true || Str'
expect_err 'TypeError' 'false || new Void'


describe 'Boolean Not'

expect_expr_bool 'false' '!true'
expect_expr_bool 'true' '!false'
expect_expr_bool 'true' '!!true'
expect_expr_bool 'false' '!!false'
expect_expr_bool 'false' '!(true && true)'
expect_expr_bool 'true' '!(true && false)'
expect_expr_bool 'true' '!(false && true)'
expect_err 'TypeError' '!1'
expect_err 'TypeError' '!""'