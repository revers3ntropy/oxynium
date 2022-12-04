describe 'Loops'

expect '
    var i = 0;
    for {
        i = i + 1;
        print_int(i);
        break;
    }
' '1'

expect '
    for {
        print("hello");
        break;
    };
    for {
        print("there");
        break;
    };
' $'hello\rthere'

expect '
    var i = 0;
    for {
        i = i + 1;
        if i < 5 {
            continue;
        };
        print_int(i);
        break;
    };
' $'5'

expect '
    const n = 9;
    var i = 0;
    for {
        i = i + 1;
        print_int(i);
        print("");
        if i > n {
            break;
        };
    };
' $'1\r2\r3\r4\r5\r6\r7\r8\r9\r10\r'