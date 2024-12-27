describe 'Class Field Assignment'

expect_err 'UnknownSymbol' 'a.a = 2'
expect_err 'SyntaxError' 'a.a ='
expect_err 'SyntaxError' '.a ='
expect_err 'SyntaxError' '.a = 1'
expect '1 2 3' '
class A {
    a: Int
}

def main () {
    let a = new A { a: 1 }
    print(a.a.Str(), " ")
    a.a = 2
    print(a.a.Str(), " ")

    print((new A { a: 1 }.a = 3).Str())
}
'
expect_err 'TypeError' '
class A {
    a: Int
}

def main () {
    let a = new A { a: 1 }
    a.b = 2
}
'
expect '1 2 2 3 2' '
class A {
    a: Int,

    def set_a(self, a: Int) {
        self.a = a
    }

    def clone(self) -> new A { a: self.a }
}

def main () {
    let a = new A { a: 1 }
    print(a.a.Str(), " ")
    a.set_a(2)
    print(a.a.Str(), " ")

    let b = a.clone()
    print(b.a.Str(), " ")
    b.a = 3
    print(b.a.Str(), " ")
    print(a.a.Str())
}
 '