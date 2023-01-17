describe 'Class Generics'

expect '1Hi1 | MyClass<Int> | MyClass<Str> | MyClass<MyClass<Int>>' '
    class S <T> { x: T }

    class A <T> {
        x: T
    }
    class B {
        b: Int
    }

    class MyClass <T> {
        fn do_something(self) {}
    }

    fn main () {
        print((new S <Int> { x: 1 }).x.str());
        print((new S <Str> { x: "Hi" }).x.str());

        let a = new A <B> {
            x: new B { b: 1 }
        };
        print(a.x.b.str());
        print(" | ");
        print(typeof (new MyClass<Int> {}));
        print(" | ");
        println(typeof (new MyClass<Str> {}));
        print(" | ");
        println(typeof (new MyClass<MyClass<Int>> {}));
    }
'
expect_err 'TypeError' '
    class C <T> {
        x: T
    }
    new C <Int> { x: "Hi" }
'
expect_err 'TypeError' '
    class C <T> {
        x: T
    }
    new C<C<Int>> {
       x: new C<Str> {
          x: "Hi"
      }
    }
'
expect_err 'TypeError' '
    class C <T> {
        x: T
    }
    new C<C<C<Int>>> {
        x: new C<C<Int>> {
           x: new C<Str> {
              x: "Hi"
          }
       }
    }
'
expect '1' '
    class C <T> {
        x: T
    }
    print(new C<C<C<Int>>> {
        x: new C<C<Int>> {
           x: new C<Int> {
              x: 1
          }
       }
    }.x.x.x.str())
'
expect_err 'UnknownSymbol' '
    class C <T> {}
    new C <T> { }
'
expect_err 'UnknownSymbol' '
    class C <T> {}
    fn a(t: T) {}
'
expect_err 'TypeError' '
    class C <T> {
        fn a(i: Int) {}
    }
    C.a(1);
'