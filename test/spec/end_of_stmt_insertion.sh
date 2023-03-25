describe 'Inserts End-Of-Statements Correctly'

expect '3' '
    const a = 1
    const b = 2
    print(
        (
        a
        +
        b
        )
        .
        Str
        (
        )
    )
    if
        a
        >
        b
    {

    }
    else
    {}

while

        a

        <

        b

    {

    break

    }

    class A

    class B
    class C {
        def a () {}
        extern def + (self, other: C) Option<Int>
    }
    class D

    extern def f() Option<Int>

    def g() {
        let a = 1 >
                2
              + 3
    }

    // this is a commend
    extern def h() Option<Int>
    //
    extern def i() Void?
    // another comment
    def my_func () {
        let mut a: Int?
        let mut b: Option<Int>
        a = Option.some!<Int>(1)
    }
'