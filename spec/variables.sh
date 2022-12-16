describe 'Accessing Global Constants'

expect_expr_bool 'true' 'true'
expect_expr_bool 'false' 'false'


describe 'Defining Global Constants'

expect '1' '
    const a = 1;
    print_int(a)
'
expect '3' '
    const a = 1;
    const b = 2;
    print_int(a + b);
'
expect 'Some String' '
    const a = "Some String";
    print(a)
'
expect_err 'TypeError' '
    const a = 1;
    const a = 2;
'


describe 'Defining Global Variables'

expect '7' '
  var a = 1;
  var b = 6;
  print_int(a + b);
'
expect '41' '
    const a = 1;
    print_int(a * 4);
    print_int(a);
'
expect_err 'TypeError' 'print_int = 1'
expect_err 'SyntaxError' 'true = 1'
expect_err 'SyntaxError' 'true = false'
expect_err 'SyntaxError' 'true = true'
expect_err 'TypeError' 'const a = 1; a = 1'
expect_err 'TypeError' '
    var a = 1;
    var a = 2;
'


describe 'Mutating Variables'

expect '2' '
    var a = 1;
    a = 2;
    print_int(a);
'
expect '9' '
    var a = 1;
    a = 5;
    a = a + 4;
    print_int(a);
'
expect_err 'TypeError' '
    var a = 1;
    a = true;
'
expect_err 'TypeError' '
    var a = 1;
    a = "hi";
'
expect_err 'TypeError' '
    var a = "";
'


describe 'Local Variables'

expect '1' '
    fn f() {
        let a = 1;
        print_int(a);
    };
    f();
'
expect '1' '
    var a = 2;
    fn f() {
        let a = 1;
        print_int(a);
    };
    f();
'
expect '42' '
    var a = 2;
    fn f() {
        let mut a = 1;
        a = 4;
        print_int(a);
    };
    f();
    print_int(a);
'
expect_err 'TypeError' '
    fn f() {
        let a = 1;
        let a = 2;
    };
'
expect_err 'TypeError' '
    fn f() {
        let mut a = 1;
        let a = 2;
    };
'
expect_err 'TypeError' '
    fn f() {
        let a = 1;
        let mut a = 2;
    };
'
expect_err 'SyntaxError' '
    fn f() { var a = 0; };
'
expect_err 'SyntaxError' '
    fn f() { const a; };
'
expect_err 'SyntaxError' '
    fn f() { var a; };
'
expect_err 'SyntaxError' '
    fn f() {
        const a = 0;
    };
'
expect_err 'SyntaxError' '
    let a = 1;
'
expect_err 'SyntaxError' '
    let mut a = 1;
'
expect_err 'TypeError' '
    fn f() {
        let a = 1;
        a = 2;
    };
'
expect_err 'TypeError' '
    fn f() {
        let mut a = 1;
        a = "";
    };
'
expect '4' '
    fn f(n: Int): Int {
        let mut a = n*n + 3 * n;
        a = a - 1;
        return a / 2;
    };
    print_int(f(2));
'
expect '4' '
    fn f(n: Int): Int {
        let mut a: Int;
        if n == 3 {
            a = 2;
        } else {
            a = 1;
        };
        return a * 2;
    };
    print_int(f(3));
'
expect '5' '
    fn f(n: Int): Int {
        if true {
            let a = n + 3;
            return a;
        };
        return n;
    };
    print_int(f(2));
'
expect_err 'UnknownSymbolError' '
    fn f(): Int {
        return a;
        let a = 5;
    };
'
expect_err 'SyntaxError' '
    fn f(): Int {
        let a: Int;
    };
'
expect_err 'TypeError' '
    fn f(): Int {
        let mut a: Int;
        return a;
    };
'
expect '1' '
    fn f(): Int {
        let mut a: Int;
        a = 1;
        return a;
    };
    print_int(f());
'


describe 'Invalid Variable Names'

expect_err 'SyntaxError' 'var 1 = 1'
expect_err 'SyntaxError' 'var 1a = 1'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'var _$_ = 1'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'var _$_a = 1'
expect_err 'SyntaxError' 'var mut = 1'
expect_err 'SyntaxError' 'var fn = 1'

expect '' 'var _ = 1'
expect '' 'var __ = 1'
expect '' 'var __hi = 1'
expect '' 'var __hi__ = 1'
expect '' 'var a$j = 1'
# Cannot start with '_$'
expect_err 'SyntaxError' 'var _$ = 1'
expect_err 'SyntaxError' 'var _$0 = 1'
# Cannot start with '$'
expect_err 'SyntaxError' 'var $1 = 1'
expect_err 'SyntaxError' 'var $ = 1'
expect_err 'SyntaxError' 'var $$ = 1'
expect_err 'SyntaxError' 'var $_ = 1'

expect_err 'SyntaxError' 'const 1 = 1'
expect_err 'SyntaxError' 'const 1a = 1'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'const _$_ = 1'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'const _$_a = 1'
expect_err 'SyntaxError' 'const mut = 1'
expect_err 'SyntaxError' 'const fn = 1'

expect_err 'SyntaxError' 'fn f() { let 1 = 1; }'
expect_err 'SyntaxError' 'fn f() { let 1a = 1; }'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'fn f() { let _$_ = 1; }'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'fn f() { let _$_a = 1; }'
expect_err 'SyntaxError' 'fn f() { let mut = 1; }'
expect_err 'SyntaxError' 'fn f() { let fn = 1; }'

expect_err 'SyntaxError' 'fn f() { let mut 1 = 1; }'
expect_err 'SyntaxError' 'fn f() { let mut 1a = 1; }'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'fn f() { let mut _$_ = 1; }'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'fn f() { let mut _$_a = 1; }'
expect_err 'SyntaxError' 'fn f() { let mut mut = 1; }'
expect_err 'SyntaxError' 'fn f() { let mut fn = 1; }'


describe 'Variables get default value on definition'

expect '22' '
    var i = 0;
    for {
        var a = 1;
        a = a + 1;
        print_int(a);
        if i >= 1 { break };
        i = i + 1;
    };
'
expect '22' '
    var i = 0;
    for {
        const a = 1;
        print_int(a+1);
        if i >= 1 { break };
        i = i + 1;
    };
'
