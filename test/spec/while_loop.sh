describe 'While Loop'

expect '1' '
    def main () {
        let mut i = 0;
        while {
            i = i + 1;
            print(i.Str());
            break;
        }
    }
'
expect '' '
    def main () {
        let mut i = 0;
        while {
            i = i + 1;
            i.Str();
            break;
        }
    }
'
expect '' '
    def main () {
        let mut i = 0;
        while {
            i = i + 1;
            i.max(1);
            break;
        }
    }
'
expect '1' '
    def main () {
        let mut i = 0;
        while {
            i = i + 1;
            print("1");
            break;
        }
    }
'
expect '' '
    def main () {
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
    def main () {
        let mut i = 0;
        while {
            i = i + 1;
            if i < 5 {
                continue;
            };
            print(i.Str());
            break;
        }
    }
'
expect '234 468 6912' '
    def main () {
        let mut i = 1;
        let mut j = 1;
        while {
            j = 1;
            while {
                j = j + 1;
                print((i*j).Str());
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
    def main () {
        let mut i = 0;
        while i < n {
            print(i.Str());
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
    }
'
expect 'hi' '
    def main () {
        let mut i = 0
        while i == 0 -> i = 1
        print("hi")
    }
'
