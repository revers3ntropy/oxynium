describe 'Class Declarations'

expect '' '
    class S {};
    class MyClass;
    def do_nothing(s: S) {};

    class TrailingCommaInClassDeclarations {
        x: Int,
    }
    class TrailingCommaAgain {
        def f(self,) {},
    }
    class _ {
        def f(self,) {},
        x: Int,
    }
    class __ {
        x: Int,
        def f(self, a: Int,) {},
    }

    class S1 {
        x: Int,
        y: Int
    }
    class A1 {
        a: A1
    }
    def main () {
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
    def main () {
        class C
    }
    def f(a: C);
'
expect_err 'UnknownSymbol' '
    def f(a: C);
    def main () {
        class C
    }
'
expect_err 'SyntaxError' '
    class A {
        class B {}
    }
'
