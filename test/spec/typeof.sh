describe 'typeof'

expect 'StrStrIntVoidTypeTypeTypeTypeCBoolStrStr'  '
    print(typeof("abc"));
    print(typeof "abc");
    print(typeof 2);
    print(typeof new Void);
    print(typeof Void);
    print(typeof Int);
    print(typeof Str);
    class C;
    print(typeof C);
    print(typeof new C);
    print(typeof true);
    print(typeof typeof Bool);
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