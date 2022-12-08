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
expect_err 'TypeError' 'true = 1'
expect_err 'TypeError' 'true = false'
expect_err 'TypeError' 'true = true'
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