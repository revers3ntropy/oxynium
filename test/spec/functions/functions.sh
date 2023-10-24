describe 'Defining Functions'

expect '12345' '
    def g() {
        print("3");
    };

    def f() {
        print("2");
        g();
        print("4");
    };

    print("1");
    f();
    print("5");
'
expect '3' '
    def g() {
        2; // just push a value to the stack
        // Ensures the stack is cleared after returns
    };
    def f() {
        1;
        g();
        print(3.Str());
        3;
    };
    f();
'
expect 'Hello' '
    def log(msg: Str) {
        print(msg);
    };
    log("Hello");
'
expect '22' '
      def sum_and_log(a: Int, b: Int, c: Int) {
          print((a + b + c).Str());
      };
      sum_and_log(5, 8, 9);
'
expect 'Hello World!' '
    def log(msg1: Str, msg2: Str, msg3: Str) {
        print(msg1);
        print(msg2);
        print(msg3);
    };
    log("Hello", " World", "!");
'
expect_err 'TypeError' '
    def f(a: Int) {
        a = 2;
    };
f(1);
'
