describe 'Function Declarations'

expect 'fn a()' ''
expect 'fn a(a: Int, b: Bool, c: Str)' ''
expect 'fn a(): Void' ''
expect 'fn a(a: Int): Str' ''
expect_err 'fn a(a): Str' 'SyntaxError'
expect_err 'fn a(a)' 'SyntaxError'
expect_err 'fn()' 'SyntaxError'
expect_err 'fn 0()' 'SyntaxError'
expect_err 'fn 0g()' 'SyntaxError'
expect_err 'fn g(); fn g();' 'TypeError'


describe 'Writing Functions'

expect '
fn g() {
    print("3");
};

fn f() {
    print("2");
    g();
    print("4");
};

print("1");
f();
print("5");
' $'1\r2\r3\r4\r5\r'