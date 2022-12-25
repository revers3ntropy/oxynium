describe 'Local Variables'

expect_err 'SyntaxError' '
    fn f() Int {
        let a: Int;
    };
'
expect '1' '
    fn f() {
        let a = "1";
        print(a);
    };
    f();
'
expect '1' '
    const a = 2;
    fn f() {
        let a = 1;
        print(a.str());
    };
    f();
'
expect '42' '
    const a = 2;
    fn f() {
        let mut a = 1;
        a = 4;
        print(a.str());
    };
    f();
    print(a.str());
'
expect_err 'TypeError' '
    const a = "";
    fn f() {
        let a = 1;
        print(a);
    };
    f();
'


describe "Don't Allow Redeclaration"

expect_err 'TypeError' '
    fn main() {
        let a = 1;
        let a = 2;
    };
'
expect_err 'TypeError' '
    fn f() {
        let mut a = 1;
        let a = 2;
    };
'
expect_err 'TypeError' '
    fn f() {
        let a = 1;
        let mut a = 2;
    };
'
expect_err 'TypeError' '
    fn f() {
        let a = 1;
        a = 2;
    };
'
expect_err 'TypeError' '
    fn main () {
        let a = 1;
        if true {
            let a = 2;
        }
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
    fn main() {
        let mut a: Int = 0;
        a = 1;
        print(a.str());
    };
'
expect_err 'TypeError' '
    fn f() {
        let mut a = 1;
        a = "";
    };
'
expect '4' '
    fn f(n: Int) Int {
        let mut a = n*n + 3 * n;
        a = a - 1;
        return a / 2;
    };
    print(f(2).str());
'
expect '4' '
    fn f(n: Int) Int {
        let mut a: Int;
        if n == 3 {
            a = 2;
        } else {
            a = 1;
        };
        return a * 2;
    };
    print(f(3).str());
'
expect '5' '
    fn f(n: Int) Int {
        if true {
            let a = n + 3;
            return a;
        };
        return n;
    };
    print(f(2).str());
'
expect '4' '
    fn main() {
        let a: Int = 4;
        print(a.str());
    };
'
expect_err 'TypeError' '
    fn main() {
        let a: Int = "";
    };
'
expect_err 'TypeError' '
    fn main() {
        let mut a: Int = "";
    };
'
expect_err 'TypeError' '
    fn main() {
        let mut a: Int = 0;
        a = ""
    };
'


describe 'Empty Local Var Declarations'

expect_err 'TypeError' '
    fn f() Int {
        let mut a: Int;
        return a;
    };
'
expect '1' '
    fn f() Int {
        let mut a: Int;
        a = 1;
        return a;
    };
    print(f().str());
'


describe 'Invalid Variable Names'

expect_err 'SyntaxError' 'const 1 = 1'
expect_err 'SyntaxError' 'const 1a = 1'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'const _$_ = 1'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'const _$_a = 1'
expect_err 'SyntaxError' 'const mut = 1'
expect_err 'SyntaxError' 'const fn = 1'

expect_err 'SyntaxError' 'fn f() { let 1 = 1; }'
expect_err 'SyntaxError' 'fn f() { let 1a = 1; }'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'fn f() { let _$_ = 1; }'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'fn f() { let _$_a = 1; }'
expect_err 'SyntaxError' 'fn f() { let mut = 1; }'
expect_err 'SyntaxError' 'fn f() { let fn = 1; }'

expect_err 'SyntaxError' 'fn f() { let mut 1 = 1; }'
expect_err 'SyntaxError' 'fn f() { let mut 1a = 1; }'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'fn f() { let mut _$_ = 1; }'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'fn f() { let mut _$_a = 1; }'
expect_err 'SyntaxError' 'fn f() { let mut mut = 1; }'
expect_err 'SyntaxError' 'fn f() { let mut fn = 1; }'


describe 'Invalid Reassignment'

expect_err 'TypeError' 'const a = 1; a = 2'
expect_err 'TypeError' 'Str = 1'
expect_err 'TypeError' 'class C; C = 1'
expect_err 'TypeError' 'Int = 1'
expect_err 'TypeError' 'print = 1'