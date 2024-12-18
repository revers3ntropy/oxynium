describe 'Generic Functions'

expect '1,hello,true' '
    def a <T> (a: T) T {
        return a
    }
    print(a!<Int>(1).Str());
    print(",");
    print(a!<Str>("hello").Str());
    print(",");
    print(a!<Bool>(true).Str());

    def trailing_comma_in_generic_params <T,> (a: T) T {
        return a
    }
'
expect_err 'TypeError' '
    def a <T> (a: T) T {
        return a
    }
    a!<Int>("");
'
expect_err 'UnknownSymbol' '
    def a <T> (a: T) T {
        return a
    }
    a!<T>("");
'
expect_err 'TypeError' '
    def a <T> (a: T) T {
        return a
    }
    a("")
'
expect_err 'TypeError' '
    def a <T> (a: T) T {
        return a
    }
    a!<Str, Str>("");
'


describe 'Generic Methods'

expect '1' '
    class C {
        def a <T> (self, t: T) T {
            return t
        }
    }
    print(new C.a!<Int>(1).Str());
'
expect '1' '
    class C {
        def a <T> (t: T) T {
            return t
        }
    }
    print(C.a!<Int>(1).Str());
'
expect '1,Hi' '
    class C <A> {
        def a <T> (self, a: A, t: T) T {
            return t
        }
    }
    print(new C<Int>.a!<Int>(1, 1).Str());
    print(",");
    print(new C<Int>.a!<Str>(1, "Hi"));
'
expect_err 'TypeError' '
    class C <A> {
        def a <T> (self, a: A, t: T) T {
            return t
        }
    }
    new C<Int>.a!<Int>(1, "hi");
'
expect_err 'TypeError' '
    class C <A> {
        def a <T> (self, a: A, t: T) T {
            return t
        }
    }
    new C<Int>.a!<Int>("hi", 1);
'
expect_err 'UnknownSymbol' '
    class C <A> {
        def a <T> (self, a: A, t: T) T {
            return t
        }
    }
    new C<Int>.a!<T>(1, 1);
'
expect_err 'UnknownSymbol' '
    class C <A> {
        def a <T> (self, a: A, t: T) T {
            return t
        },
        def b (t: T) {}
    }
'
expect_err 'TypeError' '
    class C <A> {
        def a <T> (self, a: A, t: T) T {
            return t
        }
    }
    new C<Int>.a(1, 1);
'
expect_err 'TypeError' '
    class C <A> {
        def a <T> (self, a: A, t: T) T {
            return t
        }
    }
    new C<Int>.a!<Int, Int>(1, 1);
'
expect_err 'TypeError' '
    class C <A> {
        def a <T> (self, a: A, t, T) T {
            return t
        }
    }
    new C<Int>.a!<Int, Int>(1, 1, 1);
'
expect '1,Hi' '
    class C <A, B> {
    	def a <X, Y> (self, a: A, x: X, y: Y) X {
    		return x
    	}
    }
    print(
        new C<Int, C<Int, Void>>
            .a!<Int, Str>(2, 1, "ho")
            .Str()
    );
    print(",");
    print(
    	new C <
    		Str,
    		C<
                Option<C<
                    Option<C<Void, Int>>,
                    Int
                >>,
                Void
            >
    	>
    		.a!<Str, Ptr<Int>>(
    			"hi",
    			"Hi",
    			Ptr.make!<Int>(6)
    		)
    );
'
expect_err 'TypeError' '
    class C <A> {
        // same generic param name as for class - not allowed
        def a <A> (self, a: A) A {
            return a
        }
    }
    print(new C<Int>.a!<Str>(1).Str());
'


describe 'Generic Method edge cases'

expect '1 hi' '
    class C {
        def a <T> (self, t: T) -> t,
        def b <T> (self, t: T) T -> t
    }
    print(new C.a!<Int>(1).Str(), " ")
    print(new C.b!<Str>("hi"))
'

expect 'A<Int> A<Str> A<Char>' '
    class A <T> {
        def b <U> (self) A<U> -> new A<U>
    }
    print(typeof new A<Int>, " ")
    print(typeof (new A<Int>).b!<Str>(), " ")
    print(typeof (new A<Int>).b!<Str>().b!<Char>())
'
expect 'A<Void> A<Int>' '
    class A <T> {
        def b <U> (self) A<U> -> new A<U>
    }
    print(typeof (new A<Int>).b!<Str>().b!<Char>().b!<Void>(), " ")
    print(typeof (new A<Int>).b!<Str>().b!<Char>().b!<Int>())
'
expect 'List<Int> List<Str> List<Char>' $'
    print(typeof List.empty!<Int>(), " ")
    print(typeof List.empty!<Int>().map!<Str>(fn (a: Int, b: Int) -> " "), " ")
    print(typeof List.empty!<Int>()
            .map!<Str>(fn (a: Int, b: Int) -> " ")
            .map!<Char>(fn (a: Str, b: Int) -> \' \')
    )
'
expect 'Char' $'
    print(typeof List.empty!<Int>()
            .map!<Str>(fn (a: Int, b: Int) -> " ")
            .map!<Char>(fn (a: Str, b: Int) -> \' \')
            .at_raw(0)
    )
'
expect 'Char' '
    print(typeof List.empty!<Int>()
            .map!<Str>(fn (a: Int, b: Int) -> " ")
            .map!<List<Char>>(fn (a: Str, b: Int) -> List.empty!<Char>())
            .at_raw(0).at_raw(0),
    )
'
expect 'C<Int, Str, Char> A<Char> B<Int, Void> B<B<Str, A<Int>>, C<Void, Void, Int>>' '
    class A<T> {
        def a<U>(self) B<T, U> -> new B<T, U>
    }
    class B<T, U> {
        def b<V>(self) C<T, U, V> -> new C<T, U, V>
    }
    class C<T, U, V> {
        def c(self) A<V> -> new A<V>,
        def d<Q>(self) B<T, Q> -> new B<T, Q>
    }
    print(typeof (new A<Int>).a!<Str>().b!<Char>(), " ")
    print(typeof (new A<Int>).a!<Str>().b!<Char>().c(), " ")
    print(typeof (new A<Int>).a!<Str>().b!<Char>().d!<Void>(), " ")
    print(typeof (new A<B<Str, A<Int>>>).a!<C<Void, Void, Int>>())
'
expect 'Builder<Char> Builder<Char> Builder<Char> Builder<Char> Builder<Char> Builder<Int>' '
    class Builder<T> {
        def a() Builder<Char> -> new Builder<Char>,
        def b(self) Builder<T> -> new Builder<T>,
        def c(self) -> new Builder<T>,
        def d(self) -> new Builder<T>,
        def e(self) Builder<Int> -> new Builder<Int>
    }

    print(typeof Builder.a(), " ")
    print(typeof Builder.a().b(), " ")
    print(typeof Builder.a().b().b(), " ")
    print(typeof Builder.a().c().b(), " ")
    print(typeof Builder.a().b().c().d(), " ")
    print(typeof Builder.a().b().c().d().e())
'