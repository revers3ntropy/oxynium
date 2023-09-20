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
