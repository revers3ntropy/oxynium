describe 'Class Declarations'

expect '' 'class S {};'
expect '' 'class MyClass'
expect_err 'SyntaxError' 'class class'
expect_err 'SyntaxError' 'class _$_MyClass'
expect '' '
    class S {};
    func do_nothing(s: S) {};
'
expect '' '
    class S {
        x: Int,
        y: Int
    };
'
expect '' '
    class S { s: S };
'
expect_err 'TypeError' '
    class S {};
    class S {};
'
expect_err 'TypeError' '
    class Bool {};
'
expect '' '
    func main () {
        class C;
    }
'
expect_err 'UnknownSymbol' '
    func main () {
        class C
    }
    func f(a: C);
'
expect_err 'UnknownSymbol' '
    func f(a: C);
    func main () {
        class C
    }
'
expect_err 'SyntaxError' '
    class A {
        class B {}
    }
'
