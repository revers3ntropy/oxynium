describe 'Invalid Declarations'

expect_err 'TypeError' '
    def f() Int {
        return a;
        let a = 5;
    };
'
expect_err 'TypeError' '
    def f() {
        let b = a + 5;
        let a = 5;
    };
'
expect_err 'TypeError' '
    class A;
    class A;
'
expect_err 'TypeError' '
    primitive A;
    class A;
'
expect_err 'TypeError' '
    primitive A;
    primitive A;
'
expect_err 'TypeError' '
    primitive Str;
'
expect_err 'TypeError' '
    class Str;
'
expect_err 'TypeError' '
    primitive Int;
'
expect_err 'TypeError' '
    class Int;
'


describe 'Out of Order Types'

expect '' '
    def f(a: A) A {
        return a
    }
    class A;
'
expect_err 'UnknownSymbol' '
    def f(a: A) A {
        return a
    }
'
expect '' '
    def f(a: A) B {
        return a.b
    }
    class A {
        b: B
    }
    class B
'
expect '' '
    def f(a: A) C {
        return a.b.c
    }
    class A {
        b: B
    }
    class B {
        c: C
    }
    class C
'
expect '' '
    def f(a: A) D {
        return a.b.c.d
    }
    class A {
        b: B
    }
    class B {
        c: C
    }
    class C {
        d: D
    }
    class D
'
expect '' '
    f(new A { b: new B { c: new C }});
    def f(a: A) C {
        return a.b.c.get_a(a).b.c
    }
    class A {
        b: B,
    }
    class C {
        def get_a(self, a: A) A {
            return a
        }
    }
    class B {
        c: C,
        extern def get_a(self, a: A) A,
    }
'


describe 'Undefined Access'

expect_err 'UnknownSymbol' 'a'
expect_err 'TypeError' 'const a = 0; a.b'
expect_err 'TypeError' 'const a = 0; a.b()'
expect_err 'TypeError' 'const a = 0; a.b.c'
expect_err 'TypeError' 'const a = 0; a.b.c()'
expect_err 'UnknownSymbol' 'a.b.c.d'
expect_err 'TypeError' 'const a = 0; a.b().c.d.e'
expect_err 'TypeError' '
    const a =  1;
    a.b().c.d.e()
'
expect_err 'TypeError' '
    def main() {
        let mut a: Int;
        a.b().c.d.e()
    }
'
expect_err 'TypeError' '
    def main() {
        let mut a = 0;
        a.b().c.d.e()
    }
'
expect_err 'UnknownSymbol' '
    def main() {
        a.b().c.d.e()
    }
'
expect_err 'TypeError' '
    def main() {
        main.a
    }
'
expect_err 'TypeError' '
    def f(a: Str) {
        a.some_key_that_doesnt_exist
    }
'
expect_err 'TypeError' '
    Str.some_key_that_doesnt_exist
'
expect_err 'TypeError' '
    new Str.some_key_that_doesnt_exist
'