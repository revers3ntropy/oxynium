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

expect_err 'if' 'SyntaxError'
expect_err 'if ()' 'SyntaxError'
expect_err 'if {}' 'SyntaxError'
expect_err 'if { print(); }' 'SyntaxError'
expect_err 'if { print(); }' 'SyntaxError'
expect_err 'if true { 1 } if true { 1 }' 'SyntaxError'