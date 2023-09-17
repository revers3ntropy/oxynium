describe 'Anonymous Functions'

expect '' '
    def main () {
      let a = def () {}
    }
'
expect '' '
    def main () {
      let a = def () {
        print("hello")
      }
    }
'
expect 'hello' '
    def main () {
      let a = def () {
        print("hello")
      }
      a()
    }
'
expect_err 'UnknownSymbol' '
    def main () {
      let a = 1

      let b = def () int {
        return b
      }
    }
'
expect '4' '
    def main () {
      let double = def (a: Int) Int {
        return a * 2
      }
      print(double(2).Str())
    }
'
expect '5' '
    def main () {
      let five = def () Int {
        return 5
      }
      print(five().Str())
    }
'
expect_err 'UnknownSymbol' '
    def main () {
      let five = def () Int {
        return 5
      }
      let num = def () Int {
        return five()
      }
    }
'
expect '' '
    def main () {
      def () {
        return main()
      }
    }
'
expect '' '

    def g () {}

    def main () {
      def () {
        g()
      }
    }
'
expect_err 'SyntaxError' '
    (def () {
        print("hi");
    })();
'