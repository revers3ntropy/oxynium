describe 'If Statements'

expect '
if false {
  print("hi");
}
' ''

expect '
if true {
  print("hi");
}
' $'hi\r'
expect 'if true { 1 }' ''

expect '
if false {
  print_int(1);
} else {
  print_int(2);
}
' '2'

expect '
if true {
  print_int(1);
} else {
  print_int(2);
}
' '1'

expect '
if false {
  print("1");
} else if true {
  print("2");
}
' '2'

expect '
if false {
  print_int(1);
} else if true {
  print_int(2);
} else {
  print_int(3);
}
' '2'

expect '
if false {
  print_int(1);
} else if false {
  print_int(2);
} else {
  print_int(3);
}
' '3'

expect_err 'if' 'SyntaxError'
expect_err 'if ()' 'SyntaxError'
expect_err 'if {}' 'SyntaxError'
expect_err 'if { print(); }' 'SyntaxError'
expect_err 'if { print(); }' 'SyntaxError'
expect_err 'if true { 1 } if true { 1 }' 'SyntaxError'