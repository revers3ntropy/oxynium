describe 'Class Instance Instantiation'

expect '' '
    class A { x: Int }
    new A { x: 1 };

    class B {
        x: Int,
        y: Bool,
    };
    new B { x: 1, y: true };
    new B { x: 1, y: true, };
'
expect_err 'TypeError' '
    class S { x: Int };
    class S2 { s: S };
    new S2 { s: new S { x: "hi", }, };
'
expect_err 'TypeError' '
    class S { x: Int };
    new S { };
'
expect_err 'TypeError' '
    class S { x: Int };
    new S { y: 1 };
'
expect_err 'TypeError' '
    class S { x: Int };
    new S { x: "hi" };
'
expect_err 'TypeError' '
    class S { x: Int };
    new S { x: 1, y: 2 };
'
expect_err 'TypeError' '
    class S { x: Int, y: Str };
    new S { x: 1, y: 2 };
'
expect_err 'TypeError' '
    class S { x: Int, y: Str, z: Int };
    new S { x: 1, y: "" };
'
expect_err 'UnknownSymbol' 'new s'
expect_err 'SyntaxError' 'new 1'
expect_err 'SyntaxError' 'new ""'
expect_err 'SyntaxError' 'new new C'
expect_err 'SyntaxError' 'new C()'

expect '1' '
    class C {
        a: Int
    }
    def main () {
        let a = 1;
        print(new C { a }.a.Str());
    }
'