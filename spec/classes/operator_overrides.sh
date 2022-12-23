describe 'Operator Overloads'

expect '341' '
    class Foo {
        x: Int,
        fn + (self, other: Foo) Foo {
            return new Foo {
                x: self.x + other.x
            }
        }
    }
    class Bar {
        x: Int,
        fn + (self, other: Int) Bar {
            return new Bar {
                x: self.x + other
            }
        }
        fn - (self, other: Foo) Bar {
            return new Bar {
                x: self.x - other.x
            }
        }
    }
    fn main() {
        let a = new Foo { x: 1 };
        let b = new Foo { x: 2 };
        print((a + b).x.str());
        print((new Bar { x: 1 } + 3).x.str());
        print((new Bar { x: 2 } - a).x.str());
    }
'
expect_err 'TypeError' '
    class C {
        fn + (self) C {
            return new C
        }
    }
'
expect_err 'TypeError' '
    class C {
        fn + (self, a1: C, a2: C) C {
            return new C
        }
    }
'
expect '' '
    class C {
        fn + (self, a1: C) C {
            return new C
        }
    }
'


describe 'Do Not Allow Top Level Operator Overloads'

expect_err 'SyntaxError' '
    fn + (s: Str, other: Str) Str {
        return ""
    }
'
expect_err 'SyntaxError' '
    fn + ()
'
expect_err 'SyntaxError' '
    extern fn + ()
'
expect_err 'SyntaxError' '
    fn + () Str {
        return ""
    }
'


describe 'Invalid Operator Overloads'

expect_err 'SyntaxError' '
    class C {
        fn ! (self) Str {
            return ""
        }
    }
'
expect_err 'SyntaxError' '
    class C {
        fn === (self) Str {
            return ""
        }
    }
'
expect_err 'SyntaxError' '
    class C {
        fn += (self) Str {
            return ""
        }
    }
'
expect_err 'SyntaxError' '
    class C {
        fn ^ (self) Str {
            return ""
        }
    }
'
expect_err 'SyntaxError' '
    class C {
        fn 1 (self) Str {
            return ""
        }
    }
'
expect_err 'SyntaxError' '
    class C {
        fn "" (self) Str {
            return ""
        }
    }
'