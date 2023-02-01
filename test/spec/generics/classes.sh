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
        func do_something(self) {}
    }

    class TrailingCommaInGenerics <T,>;

    func main () {
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
    func a(t: T) {}
'
expect_err 'UnknownSymbol' '
    class C <T> {}
    class D <Q> {
        func a(self, t: T) {}
    }
'
expect '' '
    class C <T> {
        func a(i: Int) {}
    }
    C.a(1);
'
expect_err 'UnknownSymbol' '
    class C <T> {
        func a(t: T) T {
            return t
        }
    }
'
expect '1,2' '
    class C <T> {
        func a(self, t: T) T {
            return t
        }
    }
    print(new C<Int>.a(1).str());
    print(",");
    print(C!<Int>.a(new C<Int>, 2).str());
'
expect_err 'TypeError' '
    class C <T> {
        func a(self, t: T) T {
            return t
        }
    }
    C.a(new C<Int>, 2)
'
expect_err 'TypeError' '
    // only an error as --allow_overrides is not true
    // but used in STD so should be working...
    class C <T> {
        extern func foo(a: T) T,
    }
    func C.foo(a: T) T {
        return a
    }
'