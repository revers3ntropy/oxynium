describe 'Loops'

expect '1' '
    fn main () {
        let mut i = 0;
        for {
            i = i + 1;
            print(i.str());
            break;
        }
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
    fn main () {
        let mut i = 0;
        for {
            i = i + 1;
            if i < 5 {
                continue;
            };
            print(i.str());
            break;
        };
    }
'

expect '12345678910' '
    const n = 9;
    fn main () {
        let mut i = 0;
        for {
            i = i + 1;
            print(i.str());
            print("");
            if i > n {
                break;
            };
        };
    }
'

expect '234 468 6912' '
    fn main () {
        let mut i = 1;
        let mut j = 1;
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
    }
'