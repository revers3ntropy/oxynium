describe 'Accessing Constants'

expect 'true' '1'
expect 'false' '0'


describe 'Defining constants'

expect 'const a = 1; a' '1'
expect '
    const a = 1;
    const b = 2;
    a + b
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

expect 'var a = 1; a = 2; print_int(a)' '2'