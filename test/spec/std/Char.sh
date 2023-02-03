describe 'primitive Char'

expect '' 'print(new Char.Str())'
expect_err 'TypeError' 'print(new Char)'


describe 'def Char.Str'

# TODO: requires char literals

describe 'def Char.is_digit'

expect 'true,true,true,false,false,false,false' '
    print("0".at(0).is_digit().Str());
    print(",");
    print("4".at(0).is_digit().Str());
    print(",");
    print("9".at(0).is_digit().Str());
    print(",");
    print(" ".at(0).is_digit().Str());
    print(",");
    print("a".at(0).is_digit().Str());
    print(",");
    print("Z".at(0).is_digit().Str());
    print(",");
    print("💖".at(0).is_digit().Str());
'


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