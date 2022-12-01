describe 'Accessing Constants'

expect_expr_bool 'true' 'true'
expect_expr_bool 'false' 'false'


describe 'Defining constants'

expect 'const a = 1; print_int(a)' '1'
expect '
    const a = 1;
    const b = 2;
    print_int(a + b);
' '3'
expect '
    const a = "Some String";
    print(a)
' $'Some String\r'


describe 'Declaring Variables'

expect '
  var a = 1;
  var b = 6;
  print_int(a + b)
' '7'


describe 'Mutating Variables'

expect '
    var a = 1;
    a = 2;
    print_int(a)
' '2'

expect_err 'print_int = 1' 'TypeError'
expect_err 'true = 1' 'TypeError'

expect '
    const a = 1;
    print_int(a * 4);
    print_int(a);
' '41'