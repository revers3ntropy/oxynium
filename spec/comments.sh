describe 'Comments'

expect '
// This is a comment
print("Hello, World!")
' 'Hello, World!'

expect '
print("Hello, World!") // This is a comment
' 'Hello, World!'

expect '
// This is a comment
print("Hello, World!") // This is a comment
// This is a comment
' 'Hello, World!'

expect '
// This is a comment
print("Hello, World!"); // Another Comment
// This is a comment
print("Hello, World!")  ////
// This is a comment
' 'Hello, World!Hello, World!'

expect '
//
// This is a comment
//
' ''

expect '//' ''
expect ' // ' ''
expect ' // ' ''