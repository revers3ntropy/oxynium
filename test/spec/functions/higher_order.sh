describe 'Higher Order Functions'

expect '2,2,3' '
    def f (a: Int) Int { return a + 1 }
    // might accidentally call "a" when we mean to call "f"
    def a (a: Int) Int { return a + 2 }
    def g (a: Int) Int { return a + 2 }

    def main () {
        print(f(1).Str())
        print(",")

        let mut a = f

        print(a(1).Str())
        print(",")

        a = g

        print(a(1).Str())
    }
'

expect '4' '
    def f (a: Int) Int { return a + 1 }
    def g (a: Int) Int { return a + 2 }

    def main () {
        let a = f
        let b = g

        print(a(b(1)).Str())
    }
'

expect 'hi' '
    def main() {
        let a = fn () Str {
            return "hi"
        }
        print(a())
    }
'
