describe 'Loops'

expect '1' '
    var i = 0;
    for {
        i = i + 1;
        print(i.str());
        break;
    }
'

expect 'hello there' '
    for {
        print("hello");
        break;
    };
    for {
        print(" there");
        break;
    };
'

expect '5' '
    var i = 0;
    for {
        i = i + 1;
        if i < 5 {
            continue;
        };
        print(i.str());
        break;
    };
'

expect '12345678910' '
    const n = 9;
    var i = 0;
    for {
        i = i + 1;
        print(i.str());
        print("");
        if i > n {
            break;
        };
    };
'

expect '234 468 6912' '
    var i = 1;
    var j = 1;
    for {
        j = 1;
        for {
            j = j + 1;
            print((i*j).str());
            if j > 3 {
                break;
            };
        };

        i = i + 1;
        if i > 3 {
            break;
        };
        print(" ");
    };
'