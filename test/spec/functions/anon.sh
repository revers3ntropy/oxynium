describe 'Anonymous Functions'

expect '' '
    def main () {
      let a = fn () {}
    }
'
expect '' '
    def main () {
      let a = fn () Int : 1
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
expect_err 'TypeError' '
    def do_something(f: Fn (Int) Int) {
        print(f(2).Str())
    }
    def main () {
        do_something(fn <T>(x: T) T { return x })
    }
'
expect_err 'SyntaxError' '
    def do_something(f: Fn <T>(T) T) {}
'
expect '2' '
    def apply<T>(t: T, f: Fn (T) T) T {
        return f(t)
    }
    def main () {
        let x = 1
        let y = apply!<Int>(x, fn (x: Int) Int { return x + 1 })
        print(y.Str())
    }
'

expect '3' '
    def apply<T, A>(t: T, applier: Fn (T) A) A {
        return applier(t)
    }
    const x = 2
    print(apply!<Int, Int>(x, fn (x: Int) Int { return x + 1 }).Str())
'
expect '4' '
    def add_one (a: Int = 2) Int {
        return a + 1
    }
    def apply<T, A>(t: T, applier: Fn (T) A) A {
        return applier(t)
    }
    const x = 3
    print(apply!<Int, Int>(x, add_one).Str())
'

expect '' '
    def make_mapper<T>(a: T, pick_a = false) Fn(T) T {
        return fn (b: T) T {
            if pick_a {
                return a
            }
            return b
        }
    }
'
