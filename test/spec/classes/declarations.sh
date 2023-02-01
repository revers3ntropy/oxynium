describe 'Class Declarations'

expect '' '
    class S {};
    class MyClass;
    func do_nothing(s: S) {};

    class TrailingCommaInClassDeclarations {
        x: Int,
    }
    class TrailingCommaAgain {
        func f(self,) {},
    }
    class _ {
        func f(self,) {},
        x: Int,
    }
    class __ {
        x: Int,
        func f(self, a: Int,) {},
    }

    class S1 {
        x: Int,
        y: Int
    }
    class A1 {
        a: A1
    }
    func main () {
        class C1;
        class S1;
    }
'
expect_err 'SyntaxError' 'class class'
# shellcheck disable=SC2016
expect_err 'SyntaxError' 'class _$_MyClass'
expect_err 'TypeError' '
    class S {};
    class S {};
'
expect_err 'TypeError' '
    class Bool {};
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
