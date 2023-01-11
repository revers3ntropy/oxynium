describe 'Class Generics'

expect '1hi' '
    class S <T> { x: T }
    println((new S <Int> { x: 1 }).x.str());
    println((new S <Str> { x: "Hi" }).x.str());
'
expect '1' '
    class A <T> {
        x: T
    }
    class B {
        b: Int
    }
    fn main () {
        let a = new A <B> {
            x: new B { b: 1 }
        };
        println(a.x.b.str());
    }
'