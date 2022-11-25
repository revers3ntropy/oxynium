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