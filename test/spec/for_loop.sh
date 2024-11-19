describe 'For Loop'

expect '' '
    def main (args: List<Utf8Str>) {
        for arg in args {}
    }
'
expect '1,2,3,' '
    def main () {
        let arr = List.empty!<Int>();
        arr.push(1);
        arr.push(2);
        arr.push(3);
        for n in arr {
            print(n.Str(), ",");
        }
    }
'
expect 'a b c ' '
    def main () {
        for c in "abc" {
            print(c.Str(), " ");
        }
    }
'
expect '0a 1b 2c ' '
    def main () {
        for c, i in "abc" {
            print(i.Str() + c.Str(), " ");
        }
    }
'
expect '0true,1false,' '
    def main () {
        let arr = List.empty!<Bool>();
        arr.push(true);
        arr.push(false);
        for b, i in arr {
            print(i.Str() + b.Str(), ",");
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
expect_err 'SyntaxError' '
    def main () {
        for _$_ in "" {}
    }
'
expect_err 'SyntaxError' '
    def main () {
        for i, _$_ in "" {}
    }
'
expect_err 'SyntaxError' '
    def main () {
        for _$_, _$_ in "" {}
    }
'