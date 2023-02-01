describe 'Defining Functions'

expect '12345' '
    func g() {
        print("3");
    };

    func f() {
        print("2");
        g();
        print("4");
    };

    print("1");
    f();
    print("5");
'
expect '3' '
    func g() {
        2; // just push a value to the stack
        // Ensures the stack is cleared when the function returns
    };
    func f() {
        1;
        g();
        print(3.Str());
        3;
    };
    f();
'
expect 'Hello' '
    func log(msg: Str) {
        print(msg);
    };
    log("Hello");
'
expect '22' '
      func sum_and_log(a: Int, b: Int, c: Int) {
          print((a + b + c).Str());
      };
      sum_and_log(5, 8, 9);
'
expect 'Hello World!' '
    func log(msg1: Str, msg2: Str, msg3: Str) {
        print(msg1);
        print(msg2);
        print(msg3);
    };
    log("Hello", " World", "!");
'
expect_err 'TypeError' '
    func f(a: Int) {
        a = 2;
    };
f(1);
'
