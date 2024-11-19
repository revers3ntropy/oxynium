describe 'primitive Char'

expect 'Char' 'print(typeof new Char)'
expect 'Char' "print(typeof 'a')"

describe 'def Char.Str'

expect '' 'print(new Char.Str())'
expect 'a' "print('a'.Str())"
expect 'ab' "print('a'.Str() + 'b'.Str())"
expect '💖' "print('💖'.Str())"
expect '10' "print(#unchecked_cast(Int, '\n').Str())"
expect '10' "print(#unchecked_cast(Int, '\n').Str())"

describe 'def Char.is_digit'

expect 'true,true,true,false,false,false,false' "
    print('0'.is_digit().Str());
    print(','.Str());
    print('4'.is_digit().Str());
    print(','.Str());
    print('9'.is_digit().Str());
    print(','.Str());
    print(' '.is_digit().Str());
    print(','.Str());
    print('a'.is_digit().Str());
    print(','.Str());
    print('Z'.is_digit().Str());
    print(','.Str());
    print('💖'.is_digit().Str());
"


describe 'def Char.=='

expect 'true,false,true,true' '
    print(("a".at(0) == "a".at(0)).Str());
    print(",");
    print(("a".at(0) == "b".at(0)).Str());
    print(",");
    print(("💖".at(0) == "💖".at(0)).Str());
    print(",");
    print(("🇨🇦".at(0) == "🇦".at(0)).Str());
'


describe 'def Char.!='

expect 'false,true,false,false' '
    print(("a".at(0) != "a".at(0)).Str());
    print(",");
    print(("a".at(0) != "b".at(0)).Str());
    print(",");
    print(("💖".at(0) != "💖".at(0)).Str());
    print(",");
    print(("🇨🇦".at(0) != "🇦".at(0)).Str());
'


describe 'def Char.from_int'

expect_expr_int '' 'Char.from_int(0)'
expect_expr_int 'a' 'Char.from_int(97)'
expect_expr_int 'A' 'Char.from_int(65)'
expect_expr_int '0' 'Char.from_int(48)'
expect_expr_int ' ' 'Char.from_int(32)'
# TODO: characters above 0x7F (127) is undefined behaviour