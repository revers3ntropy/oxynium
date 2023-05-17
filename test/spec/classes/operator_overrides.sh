describe 'Operator Overloads'

expect '341' '
    class Foo {
        x: Int,
        def + (self, other: Foo) Foo {
            return new Foo {
                x: self.x + other.x
            }
        },
    }
    class Bar {
        x: Int,
        def + (self, other: Int) Bar {
            return new Bar {
                x: self.x + other
            }
        },
        def - (self, other: Foo) Bar {
            return new Bar {
                x: self.x - other.x
            }
        },
    }
    def main() {
        let a = new Foo { x: 1 }
        let b = new Foo { x: 2 }
        print((a + b).x.Str())
        print((new Bar { x: 1 } + 3).x.Str())
        print((new Bar { x: 2 } - a).x.Str())
    }
'


describe 'Do Not Allow Top Level Operator Overloads'

expect_err 'SyntaxError' '
    def + (s: Str, other: Str) Str {
        return ""
    }
'
expect_err 'SyntaxError' '
    def + ()
'
expect_err 'SyntaxError' '
    extern def + ()
'
expect_err 'SyntaxError' '
    def + () Str {
        return ""
    }
'


describe 'Invalid Operator Overloads'

expect_err 'SyntaxError' '
    class C {
        def ! (self) Str {
            return ""
        }
    }
'
expect_err 'SyntaxError' '
    class C {
        def === (self) Str {
            return ""
        }
    }
'
expect_err 'SyntaxError' '
    class C {
        def += (self) Str {
            return ""
        }
    }
'
expect_err 'SyntaxError' '
    class C {
        def ^ (self) Str {
            return ""
        }
    }
'
expect_err 'SyntaxError' '
    class C {
        def 1 (self) Str {
            return ""
        }
    }
'
expect_err 'SyntaxError' '
    class C {
        def "" (self) Str {
            return ""
        }
    }
'
expect_err 'TypeError' '
    class C {
        def + (self) C {
            return new C
        }
    }
'
expect_err 'TypeError' '
    class C {
        def + (self, a1: C, a2: C) C {
            return new C
        }
    }
'
expect_err 'TypeError' '
    class C {
        def + (self, a1: C = new C) C {
            return new C
        }
    }
'
expect '' '
    class C {
        def + (self, a1: C) C {
            return new C
        }
    }
'