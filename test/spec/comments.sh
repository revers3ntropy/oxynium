describe 'Comments'

expect 'Hello, World!' '
    // This is a comment
    print("Hello, World!")
'

expect 'Hello, World!' '
    print("Hello, World!") // This is a comment
'

expect 'Hello, World!' '
    // This is a comment
    print("Hello, World!") // This is a comment
    // This is a comment
'

expect 'Hello, World!Hello, World!' '
    // This is a comment
    print("Hello, World!"); // Another Comment
    // This is a comment
    print("Hello, World!")  ////
    // This is a comment
'

expect '' '
    //
    // This is a comment
    //
'

expect '' '//'
expect '' ' // '
expect '' ' // '