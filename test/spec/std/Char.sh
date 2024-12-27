describe 'primitive Char'

expect 'Char' 'print(typeof new Char)'
expect 'Char' "print(typeof 'a')"

describe 'def Char.Str'

expect '' 'print(new Char.Str())'
expect 'a' "print('a'.Str())"
expect 'ab' "print('a'.Str() + 'b'.Str())"
expect '💖' "print('💖'.Str())"
expect '10' "print(#unchecked_cast(Int, '\n').Str())"
expect '65' "print(#unchecked_cast(Int, 'A').Str())"

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

expect 'true,false,true,true' $'
    print((\'a\' == \'a\').Str(), ",")
    print((\'a\' == \'b\').Str(), ",")
    print((\'💖\' == \'💖\').Str(), ",")
    print(("🇨🇦".at_raw(0) == "🇦".at_raw(0)).Str())
'


describe 'def Char.!='

expect 'false,true,false,false' $'
    print((\'a\' != \'a\').Str(), ",")
    print((\'a\' != \'b\').Str(), ",")
    print((\'💖\' != \'💖\').Str(), ",")
    print(("🇨🇦".at_raw(0) != "🇦".at_raw(0)).Str())
'


describe 'def Char.from_int'

expect_expr_int '' 'Char.from_int(0)'
expect_expr_int 'a' 'Char.from_int(97)'
expect_expr_int 'A' 'Char.from_int(65)'
expect_expr_int '0' 'Char.from_int(48)'
expect_expr_int ' ' 'Char.from_int(32)'
# TODO: characters above 0x7F (127) is undefined behaviour