describe 'If Statements'

expect '2' '
    if false {
      print("1");
    }
    if true {
      print("2");
    }
'
expect '245' '
    if false -> print("1")
    if true ->
      print("2")

    // might try to insert a ; before the else!
    if false -> print("3")
    else print("4")

    if true -> print("5")
    else -> print("6")
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
      print(1.Str());
    } else {
      print(2.Str());
    }
'
expect '1' '
    if true {
      print(1.Str());
    } else {
      print(2.Str());
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
      print(1.Str());
    } else if true {
      print(2.Str());
    } else {
      print(3.Str());
    }
'
expect '3' '
    if false {
      print(1.Str());
    } else if false {
      print(2.Str());
    } else {
      print(3.Str());
    }
'

expect_err 'SyntaxError' 'if'
expect_err 'SyntaxError' 'if ()'
expect_err 'SyntaxError' 'if (false) print(2.Str());'
expect_err 'SyntaxError' 'if {}'
expect_err 'SyntaxError' 'if { print(); }'

expect_err 'TypeError' 'if 0 { 1 }'
expect_err 'TypeError' 'if "" { 1 }'
expect_err 'TypeError' 'if -100 { 1 }'
expect_err 'TypeError' 'if 1 + 1 { 1 }'
expect_err 'TypeError' 'if Str { 1 }'
expect_err 'SyntaxError' 'if true { 1 } else { 2 } else { 3 }'
