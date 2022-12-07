describe 'If Statements'

expect '' '
    if false {
      print("hi");
    }
'
expect 'hi' '
    if true {
      print("hi");
    }
'
expect '' 'if true { 1 }'
expect '2' '
    if false {
      print_int(1);
    } else {
      print_int(2);
    }
'
expect '1' '
    if true {
      print_int(1);
    } else {
      print_int(2);
    }
'
expect '2' '
    if false {
      print("1");
    } else if true {
      print("2");
    }
'
expect '2' '
    if false {
      print_int(1);
    } else if true {
      print_int(2);
    } else {
      print_int(3);
    }
'
expect '3' '
    if false {
      print_int(1);
    } else if false {
      print_int(2);
    } else {
      print_int(3);
    }
'

expect_err 'SyntaxError' 'if'
expect_err 'SyntaxError' 'if ()'
expect_err 'SyntaxError' 'if (false) print_int(2);'
expect_err 'SyntaxError' 'if {}'
expect_err 'SyntaxError' 'if { print(); }'
expect_err 'SyntaxError' 'if { print(); }'
expect_err 'SyntaxError' 'if true { 1 } if true { 1 }'

expect_err 'TypeError' 'if 0 { 1 }'
expect_err 'TypeError' 'if "" { 1 }'
expect_err 'TypeError' 'if -100 { 1 }'
expect_err 'TypeError' 'if 1 + 1 { 1 }'