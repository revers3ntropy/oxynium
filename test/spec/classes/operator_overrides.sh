describe 'Operator Overloads'

expect '341' '
    class Foo {
        x: Int,
        func + (self, other: Foo) Foo {
            return new Foo {
                x: self.x + other.x
            }
        }
    }
    class Bar {
        x: Int,
        func + (self, other: Int) Bar {
            return new Bar {
                x: self.x + other
            }
        }
        func - (self, other: Foo) Bar {
            return new Bar {
                x: self.x - other.x
            }
        }
    }
    func main() {
        let a = new Foo { x: 1 };
        let b = new Foo { x: 2 };
        print((a + b).x.str());
        print((new Bar { x: 1 } + 3).x.str());
        print((new Bar { x: 2 } - a).x.str());
    }
'


describe 'Do Not Allow Top Level Operator Overloads'

expect_err 'SyntaxError' '
    func + (s: Str, other: Str) Str {
        return ""
    }
'
expect_err 'SyntaxError' '
    func + ()
'
expect_err 'SyntaxError' '
    extern func + ()
'
expect_err 'SyntaxError' '
    func + () Str {
        return ""
    }
'


describe 'Invalid Operator Overloads'

expect_err 'SyntaxError' '
    class C {
        func ! (self) Str {
            return ""
        }
    }
'
expect_err 'SyntaxError' '
    class C {
        func === (self) Str {
            return ""
        }
    }
'
expect_err 'SyntaxError' '
    class C {
        func += (self) Str {
            return ""
        }
    }
'
expect_err 'SyntaxError' '
    class C {
        func ^ (self) Str {
            return ""
        }
    }
'
expect_err 'SyntaxError' '
    class C {
        func 1 (self) Str {
            return ""
        }
    }
'
expect_err 'SyntaxError' '
    class C {
        func "" (self) Str {
            return ""
        }
    }
'
expect_err 'TypeError' '
    class C {
        func + (self) C {
            return new C
        }
    }
'
expect_err 'TypeError' '
    class C {
        func + (self, a1: C, a2: C) C {
            return new C
        }
    }
'
expect_err 'TypeError' '
    class C {
        func + (self, a1: C = new C) C {
            return new C
        }
    }
'
expect '' '
    class C {
        func + (self, a1: C) C {
            return new C
        }
    }
'