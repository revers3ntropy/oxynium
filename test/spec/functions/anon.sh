describe 'Anonymous Functions'

expect '' '
    def main () {
      let a = fn () {}
    }
'
expect '' '
    def main () {
      let a = fn () {
        print("hello")
      }
    }
'
expect 'hello' '
    def main () {
      let a = fn () {
        print("hello")
      }
      a()
    }
'
expect_err 'UnknownSymbol' '
    def main () {
      let a = 1

      let b = fn () int {
        return b
      }
    }
'
expect '4' '
    def main () {
      let double = fn (a: Int) Int {
        return a * 2
      }
      print(double(2).Str())
    }
'
expect '5' '
    def main () {
      let five = fn () Int {
        return 5
      }
      print(five().Str())
    }
'
expect_err 'UnknownSymbol' '
    def main () {
      let five = fn () Int {
        return 5
      }
      let num = fn () Int {
        return five()
      }
    }
'
expect_err 'UnknownSymbol' '
    def main () {
        let five = 5
        fn () Int {
            return five
        }
    }
'
expect '' '
    def main () {
      fn () {
        return main()
      }
    }
'
expect '' '
    def g () {}
    def main () {
      fn () {
        g()
      }
    }
'
expect_err 'SyntaxError' '
    (fn () {
        print("hi");
    })();
'

expect '13' '
    def do_something(f: Fn () Int) {
        print(f().Str())
    }

    def main () {
        do_something(fn () Int { return 13 })
    }
'
expect '43' '
    def do_something(f: Fn (Int) Int) Int {
        return f(42)
    }

    def main () {
        let plus_one = fn (x: Int) Int {
            return x + 1
        }
        print(do_something(plus_one).Str())
    }
'
expect '43' '
    def do_something(f: Fn (Int) Int) Int {
        return f(42)
    }

    def main () {
        print(do_something(fn (x: Int) Int {
            return x + 1
        }).Str())
    }
'
