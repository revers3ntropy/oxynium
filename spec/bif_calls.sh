describe '`print` BIF'

expect 'Hello, World!' 'print("Hello, World!")'
expect_err 'TypeError' 'print(1)'
expect_err 'TypeError' 'print(true)'
expect_err 'TypeError' 'print("", "")'
expect_err 'TypeError' 'print("", true)'


describe '`print_int` BIF'

expect '1' 'print_int(1)'
expect '0' 'print_int(9*7%3)'
expect_err 'TypeError' 'print_int(true)'
expect_err 'TypeError' 'print_int("Hi")'

expect '42135' '
fn g() {
    print_int(1);
};

fn f() {
    print_int(2);
    g();
    print_int(3);
};

print_int(4);
f();
print_int(5);
'
