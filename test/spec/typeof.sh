describe 'typeof'

expect 'Str Str Int Void Void Int Str C C Bool Str Str'  '
    print(typeof("abc"), " ")
    print(typeof "abc", " ")
    print(typeof 2, " ")
    print(typeof new Void, " ")
    print(typeof Void, " ")
    print(typeof Int, " ")
    print(typeof Str, " ")

    class C
    print(typeof C, " ")
    print(typeof new C, " ")

    print(typeof true, " ")
    print(typeof typeof Bool, " ")
    print(typeof typeof typeof new Void)
'
expect_err 'UnknownSymbol' 'print(typeof Type)'
expect 'Fn a() Void,Void' '
    def a() {}
    print(typeof a);
    print(",");
    print(typeof a());
'
expect_err 'SyntaxError' 'print(typeof)'
expect_err 'SyntaxError' 'print(typeof 1 2)'
expect_err 'SyntaxError' 'print(typeof ())'
expect_err 'SyntaxError' 'print(typeof (1, 2))'
expect_err 'SyntaxError' 'print(typeof {})'
expect_err 'SyntaxError' 'print(typeof {1, 2})'
expect_err 'SyntaxError' 'print(typeof [])'
expect_err 'SyntaxError' 'print(typeof [1, 2])'
expect_err 'SyntaxError' 'print(typeof new)'
expect_err 'SyntaxError' 'print(typeof new 1)'
expect_err 'SyntaxError' 'print(typeof typeof)'
expect_err 'SyntaxError' 'print(typeof while {})'
expect_err 'SyntaxError' 'print(typeof if true {})'
expect 'Fn a() Void' 'print(typeof def a() {})'
expect 'Fn fn@in.oxy#14() Void' 'print(typeof fn () {})'

expect 'T,T,Int,Str' '
    def a <T> (a: T) T {
        print(typeof a, ",")
        print(typeof T, ",")
        return a
    }
    def main () {
        let int = a!<Int>(1)
        print(typeof int, ",")
        print(typeof a!<Str>("Hi"))
    }
'
