describe 'Loops'

expect '1' '
    fn main () {
        let mut i = 0;
        while {
            i = i + 1;
            print(i.str());
            break;
        }
    }
'
expect '' '
    fn main () {
        let mut i = 0;
        while {
            i = i + 1;
            i.str();
            break;
        }
    }
'
expect '1' '
    fn main () {
        let mut i = 0;
        while {
            i = i + 1;
            print("1");
            break;
        }
    }
'
expect '' '
    fn main () {
        let mut i = 0;
        while {
            i = i + 1;
            break;
        }
    }
'
expect 'hello there' '
    while {
        print("hello");
        break;
    }
    while {
        print(" there");
        break;
    }
'
expect '5' '
    fn main () {
        let mut i = 0;
        while {
            i = i + 1;
            if i < 5 {
                continue;
            };
            print(i.str());
            break;
        }
    }
'
expect '234 468 6912' '
    fn main () {
        let mut i = 1;
        let mut j = 1;
        while {
            j = 1;
            while {
                j = j + 1;
                print((i*j).str());
                if j > 3 {
                    break;
                };
            }

            i = i + 1;
            if i > 3 {
                break;
            }
            print(" ");
        };
    }
'
expect '012345678' '
    const n = 9;
    fn main () {
        let mut i = 0;
        while i < n {
            print(i.str());
            i = i + 1;
        }
    }
'
expect_err 'TypeError' '
    while 1 {}
'
expect_err 'TypeError' '
    while "" {}
'
expect_err 'TypeError' '
    class C;
    while C {}
'
expect_err 'TypeError' '
    class C;
    while new C {} {};
'
expect 'hi' '
    while true {
        print("hi");
        break;
    };
'