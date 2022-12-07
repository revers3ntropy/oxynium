describe 'Accessing Constants'

expect_expr_bool 'true' 'true'
expect_expr_bool 'false' 'false'


describe 'Defining constants'

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


describe 'Declaring Variables'

expect '7' '
  var a = 1;
  var b = 6;
  print_int(a + b)
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



describe 'Mutating Variables'

expect '2' '
    var a = 1;
    a = 2;
    print_int(a)
'
expect '9' '
    var a = 1;
    a = 5;
    a = a + 4;
    print_int(a)
'
expect_err 'TypeError' '
    var a = 1;
    a = true;
'
expect_err 'TypeError' '
    var a = 1;
    a = "hi";
'
