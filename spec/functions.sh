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

expect '
fn g() {
    2;
};

fn f() {
    1;
    g();
    print_int(3);
    3;
};

f();
' '3'

expect '
fn log(msg: Str) {
    print(msg);
};
log("Hello");
' $'Hello\r'

expect '
fn sum_and_log(a: Int, b: Int, c: Int) {
    print_int(a + b + c);
};
sum_and_log(5, 8, 9);
' '22'

expect '
fn log(msg1: Str, msg2: Str) {
    print(msg1);
    print(msg2);
};
log("Hello", "World");
' $'Hello\rWorld\r'