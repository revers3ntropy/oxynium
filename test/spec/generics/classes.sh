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
        def do_something(self) {}
    }

    class TrailingCommaInGenerics <T,>;

    def main () {
        print((new S <Int> { x: 1 }).x.Str());
        print((new S <Str> { x: "Hi" }).x.Str());

        let a = new A <B> {
            x: new B { b: 1 }
        };
        print(a.x.b.Str());
        print(" | ");
        print(typeof (new MyClass<Int> {}));
        print(" | ");
        print(typeof (new MyClass<Str> {}));
        print(" | ");
        print(typeof (new MyClass<MyClass<Int>> {}));
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
    }.x.x.x.Str())
'
expect_err 'UnknownSymbol' '
    class C <T> {}
    new C <T> { }
'
expect_err 'UnknownSymbol' '
    class C <T> {}
    def a(t: T) {}
'
expect_err 'UnknownSymbol' '
    class C <T> {}
    class D <Q> {
        def a(self, t: T) {}
    }
'
expect '' '
    class C <T> {
        def a(i: Int) {}
    }
    C.a(1);
'
expect_err 'UnknownSymbol' '
    class C <T> {
        def a(t: T) T {
            return t
        }
    }
'
expect '1,2' '
    class C <T> {
        def a(self, t: T) T {
            return t
        }
    }
    print(new C<Int>.a(1).Str());
    print(",");
    print(C!<Int>.a(new C<Int>, 2).Str());
'
expect_err 'TypeError' '
    class C <T> {
        def a(self, t: T) T {
            return t
        }
    }
    C.a(new C<Int>, 2)
'