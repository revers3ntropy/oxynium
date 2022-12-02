describe 'Comments'

expect '
// This is a comment
print("Hello, World!")
' $'Hello, World!\r'

expect '
print("Hello, World!") // This is a comment
' $'Hello, World!\r'

expect '
// This is a comment
print("Hello, World!") // This is a comment
// This is a comment
' $'Hello, World!\r'

expect '
// This is a comment
print("Hello, World!"); // Another Comment
// This is a comment
print("Hello, World!")  ////
// This is a comment
' $'Hello, World!\rHello, World!\r'

expect '
//
// This is a comment
//
' ''

expect '//' ''
expect ' // ' ''
expect ' // ' ''