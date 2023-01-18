describe 'If Statements'

expect '2' '
    if false {
      print("1");
    }
    if true {
      print("2");
    }
'
expect 'hi' '
    if true {
      print("hi");
    }
    if false {
      print("bye");
    }
'
expect '' 'if true { 1 }'
expect '2' '
    if false {
      print(1.str());
    } else {
      print(2.str());
    }
'
expect '1' '
    if true {
      print(1.str());
    } else {
      print(2.str());
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
      print(1.str());
    } else if true {
      print(2.str());
    } else {
      print(3.str());
    }
'
expect '3' '
    if false {
      print(1.str());
    } else if false {
      print(2.str());
    } else {
      print(3.str());
    }
'

expect_err 'SyntaxError' 'if'
expect_err 'SyntaxError' 'if ()'
expect_err 'SyntaxError' 'if (false) print(2.str());'
expect_err 'SyntaxError' 'if {}'
expect_err 'SyntaxError' 'if { print(); }'
expect_err 'SyntaxError' 'if { print(); }'

expect_err 'TypeError' 'if 0 { 1 }'
expect_err 'TypeError' 'if "" { 1 }'
expect_err 'TypeError' 'if -100 { 1 }'
expect_err 'TypeError' 'if 1 + 1 { 1 }'
expect_err 'TypeError' 'if Str { 1 }'