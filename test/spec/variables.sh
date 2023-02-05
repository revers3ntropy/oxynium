describe 'Local Variables'

expect_err 'SyntaxError' '
    def main() Int {
        let a: Int;
    }
'
expect '1' '
    def main() {
        let a = "1";
        print(a);
    }
'
expect '1' '
    const a = 2;
    def main() {
        let a = 1;
        print(a.Str());
    }
'
expect '42' '
    const a = 2;
    def f() {
        let mut a = 1;
        a = 4;
        print(a.Str());
    };
    f();
    print(a.Str());
'
expect_err 'TypeError' '
    const a = "";
    def main() {
        let a = 1;
        print(a);
    }
'


describe "Don't Allow Redeclaration"

expect_err 'TypeError' '
    def main() {
        let a = 1;
        let a = 2;
    }
'
expect_err 'TypeError' '
    def f() {
        let mut a = 1;
        let a = 2;
    }
'
expect_err 'TypeError' '
    def f() {
        let a = 1;
        let mut a = 2;
    }
'
expect_err 'TypeError' '
    def f() {
        let a = 1;
        a = 2;
    }
'
expect_err 'TypeError' '
    def main () {
        let a = 1;
        if true {
            let a = 2;
        }
    }
'
expect '' '
    def main () {
        let mut a: Int;
        if true {
            a = 2;
        } else {
            a = 3;
        }
        a = 4;
    }
'


describe "Don't Allow Local Var Dec in Global Scope"

expect_err 'SyntaxError' '
    let a = 1;
'
expect_err 'SyntaxError' '
    let mut a = 1;
'


describe 'Local Var Reassignment'

expect '1' '
    def main () {
        let mut a: Int = 0;
        a = 1;
        print(a.Str());
    }
'
expect_err 'TypeError' '
    def main () {
        let mut a = 1;
        a = "";
    }
'
expect '4' '
    def f (n: Int) Int {
        let mut a = n*n + 3 * n;
        a = a - 1;
        return a / 2;
    };
    print(f(2).Str());
'
expect '4' '
    def f (n: Int) Int {
        let mut a: Int;
        if n == 3 {
            a = 2;
        } else {
            a = 1;
        };
        return a * 2;
    };
    print(f(3).Str());
'
expect '5' '
    def f(n: Int) Int {
        if true {
            let a = n + 3;
            return a;
        };
        return n;
    };
    print(f(2).Str());
'
expect '4' '
    def main () {
        let a: Int = 4;
        print(a.Str());
    }
'
expect_err 'TypeError' '
    def main () {
        let a: Int = "";
    }
'
expect_err 'TypeError' '
    def main () {
        let mut a: Int = "";
    }
'
expect_err 'TypeError' '
    def main () {
        let mut a: Int = 0;
        a = ""
    }
'


describe 'Empty Local Var Declarations'

expect_err 'TypeError' '
    def f() Int {
        let mut a: Int;
        return a;
    };
'
expect '1' '
    def f() Int {
        let mut a: Int;
        a = 1;
        return a;
    };
    print(f().Str());
'
expect_err 'SyntaxError' '
    def f() {
        let a;
    }
'
expect_err 'SyntaxError' '
    def f() {
        let a: Int;
    }
'


describe 'Invalid Variable Names'

expect_err 'SyntaxError' 'const 1 = 1'
expect_err 'SyntaxError' 'const 1a = 1'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'const _$_ = 1'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'const _$_a = 1'
expect_err 'SyntaxError' 'const mut = 1'
expect_err 'SyntaxError' 'const def = 1'

expect_err 'SyntaxError' 'def f() { let 1 = 1; }'
expect_err 'SyntaxError' 'def f() { let 1a = 1; }'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'def f() { let _$_ = 1; }'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'def f() { let _$_a = 1; }'
expect_err 'SyntaxError' 'def f() { let mut = 1; }'
expect_err 'SyntaxError' 'def f() { let def = 1; }'

expect_err 'SyntaxError' 'def f() { let mut 1 = 1; }'
expect_err 'SyntaxError' 'def f() { let mut 1a = 1; }'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'def f() { let mut _$_ = 1; }'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'def f() { let mut _$_a = 1; }'
expect_err 'SyntaxError' 'def f() { let mut mut = 1; }'
expect_err 'SyntaxError' 'def f() { let mut def = 1; }'


describe 'Invalid Reassignment'

expect_err 'TypeError' 'const a = 1; a = 2'
expect_err 'TypeError' 'Str = 1'
expect_err 'TypeError' 'class C; C = 1'
expect_err 'TypeError' 'Int = 1'
expect_err 'TypeError' 'print = 1'


describe 'Bin Op Reassignment'

expect '2,1,4,2,1' '
    def main() {
        let mut a = 1;
        a += 1;
        print(a.Str());
        print(",");
        a -= 1;
        print(a.Str());
        print(",");
        a *= 4;
        print(a.Str());
        print(",");
        a /= 2;
        print(a.Str());
        print(",");
        a += 1;
        a %= 2;
        print(a.Str());
    }
'
expect_err 'TypeError' '
    def main() {
        let mut a = 1;
        a += "";
    }
'
expect_err 'TypeError' '
    def main() {
        let mut a = 1;
        a += "";
    }
'
expect_err 'TypeError' '
    const a = 1;
    def main() {
        a += "";
    }
'
expect 'ab' '
    def main() {
        let mut a = "a";
        a += "b";
        print(a);
    }
'
expect_err 'TypeError' '
    def main() {
        let mut a = "a";
        a -= "b";
    }
'