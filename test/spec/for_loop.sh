describe 'For Loop'

expect '' '
    def main (args: List<Utf8Str>) {
        for arg in args {}
    }
'
expect '1,2,3' '
    def main () {
        let arr = List.empty!<Int>();
        arr.push(1);
        arr.push(2);
        arr.push(3);
        for i in arr {
            print(i.Str(), "");
            print(",", "");
        }
    }
'
expect 'a b c' '
    def main () {
        for c in "abc" {
            print(c.Str(), "");
            print(" ", "");
        }
    }
'
expect '' '
    def main () {
        let arr = List.empty!<Int>();
        for i in arr {
            print(i.Str(), "");
            print(",", "");
        }
    }
'
expect_err 'TypeError' '
    def main () {
        for i in 1 {}
    }
'
expect_err 'TypeError' '
    def main () {
        for i in true {}
    }
'