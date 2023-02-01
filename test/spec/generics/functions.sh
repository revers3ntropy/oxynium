describe 'Generic Functions'

expect '1,hello,true' '
    func a <T> (a: T) T {
        return a
    }
    print(a!<Int>(1).Str());
    print(",");
    print(a!<Str>("hello").Str());
    print(",");
    print(a!<Bool>(true).Str());

    func trailing_comma_in_generic_params <T,> (a: T) T {
        return a
    }
'
expect_err 'TypeError' '
    func a <T> (a: T) T {
        return a
    }
    a!<Int>("");
'
expect_err 'UnknownSymbol' '
    func a <T> (a: T) T {
        return a
    }
    a!<T>("");
'
expect_err 'TypeError' '
    func a <T> (a: T) T {
        return a
    }
    a("")
'
expect_err 'TypeError' '
    func a <T> (a: T) T {
        return a
    }
    a!<Str, Str>("");
'


describe 'Generic Methods'

expect '1' '
    class C {
        func a <T> (self, t: T) T {
            return t
        }
    }
    print(new C.a!<Int>(1).Str());
'
expect '1' '
    class C {
        func a <T> (t: T) T {
            return t
        }
    }
    print(C.a!<Int>(1).Str());
'
expect '1,Hi' '
    class C <A> {
        func a <T> (self, a: A, t: T) T {
            return t
        }
    }
    print(new C<Int>.a!<Int>(1, 1).Str());
    print(",");
    print(new C<Int>.a!<Str>(1, "Hi"));
'
expect_err 'TypeError' '
    class C <A> {
        func a <T> (self, a: A, t: T) T {
            return t
        }
    }
    new C<Int>.a!<Int>(1, "hi");
'
expect_err 'TypeError' '
    class C <A> {
        func a <T> (self, a: A, t: T) T {
            return t
        }
    }
    new C<Int>.a!<Int>("hi", 1);
'
expect_err 'UnknownSymbol' '
    class C <A> {
        func a <T> (self, a: A, t: T) T {
            return t
        }
    }
    new C<Int>.a!<T>(1, 1);
'
expect_err 'UnknownSymbol' '
    class C <A> {
        func a <T> (self, a: A, t: T) T {
            return t
        }
        func b (t: T) {}
    }
'
expect_err 'TypeError' '
    class C <A> {
        func a <T> (self, a: A, t: T) T {
            return t
        }
    }
    new C<Int>.a(1, 1);
'
expect_err 'TypeError' '
    class C <A> {
        func a <T> (self, a: A, t: T) T {
            return t
        }
    }
    new C<Int>.a!<Int, Int>(1, 1);
'
expect_err 'TypeError' '
    class C <A> {
        func a <T> (self, a: A, t, T) T {
            return t
        }
    }
    new C<Int>.a!<Int, Int>(1, 1, 1);
'
expect '1,Hi' '
    class C <A, B> {
    	func a <X, Y> (self, a: A, x: X, y: Y) X {
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
        func a <A> (self, a: A) A {
            return a
        }
    }
    print(new C<Int>.a!<Str>(1).Str());
'
