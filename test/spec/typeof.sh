describe 'typeof'

expect 'Str Str Int Void Type Type Type Type C Bool Str Str'  '
    print(typeof("abc"));
    print(" ");
    print(typeof "abc");
    print(" ");
    print(typeof 2);
    print(" ");
    print(typeof new Void);
    print(" ");
    print(typeof Void);
    print(" ");
    print(typeof Int);
    print(" ");
    print(typeof Str);
    print(" ");

    class C;
    print(typeof C);
    print(" ");
    print(typeof new C);
    print(" ");

    print(typeof true);
    print(" ");
    print(typeof typeof Bool);
    print(" ");
    print(typeof typeof typeof new Void);
'
expect_err 'UnknownSymbol' 'print(typeof Type)'
expect 'Fn a(): VoidVoid' '
    fn a() {}
    print(typeof a);
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
expect_err 'SyntaxError' 'print(typeof fn a() {})'

expect 'T,Type,Int,Str' '
    fn a <T> (a: T) T {
        print(typeof a);
        print(",");
        print(typeof T);
        print(",");
        return a
    }
    fn main () {
        let int = a!<Int>(1);
        print(typeof int);
        print(",");
        print(typeof a!<Str>("Hi"));
    }
'