describe 'Defining Functions'

expect '12345' '
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
'
expect '3' '
    fn g() {
        2; // just push a value to the stack
        // Ensures the stack is cleared when the function returns
    };
    fn f() {
        1;
        g();
        print(3.str());
        3;
    };
    f();
'
expect 'Hello' '
    fn log(msg: Str) {
        print(msg);
    };
    log("Hello");
'
expect '22' '
      fn sum_and_log(a: Int, b: Int, c: Int) {
          print((a + b + c).str());
      };
      sum_and_log(5, 8, 9);
'
expect 'Hello World!' '
    fn log(msg1: Str, msg2: Str, msg3: Str) {
        print(msg1);
        print(msg2);
        print(msg3);
    };
    log("Hello", " World", "!");
'
expect_err 'TypeError' '
    fn f(a: Int) {
        a = 2;
    };
f(1);
'
