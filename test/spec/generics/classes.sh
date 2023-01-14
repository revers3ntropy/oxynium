describe 'Class Generics'

expect '1Hi1' '
    class S <T> { x: T }

    class A <T> {
        x: T
    }
    class B {
        b: Int
    }
    fn main () {
        println((new S <Int> { x: 1 }).x.str());
        println((new S <Str> { x: "Hi" }).x.str());

        let a = new A <B> {
            x: new B { b: 1 }
        };
        println(a.x.b.str());
    }
'
expect_err 'TypeError' '
    class C <T> {
        x: T
    }
    new C <Int> { x: "Hi" }
'
expect_err 'UnknownSymbol' '
    class C <T> {}
    new C <T> { }
'