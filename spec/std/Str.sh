describe 'Str'

expect '' 'print(new Str.str().str())'
expect '' 'print(new Str.str())'
expect '' 'print(new Str)'
expect 'Hi' 'print("Hi")'
expect_err 'SyntaxError' 'print("hi'
expect_err 'SyntaxError' '"hi'

describe 'UTF-8 Support'

expect 'ݫݨݫ' 'print("ݫݨݫ")'
expect '' 'print("")'
expect '⸻' 'print("⸻")'
expect '﷽' 'print("﷽")'
expect 'In UTF-8 The longest character is "﷽"' 'print("In UTF-8 The longest character is \"﷽\"")'
expect ' ௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌ' 'print(" ௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌௌ")'
expect 'This is a very long string literal, it has lots of characters in so it might break things like the line length limit for NASM. I think the line length limit is 255 characters so I should maybe try to get that many characters at least, but that limit might be wrong actually...' '
    print("This is a very long string literal, it has lots of characters in so it might break things like the line length limit for NASM. I think the line length limit is 255 characters so I should maybe try to get that many characters at least, but that limit might be wrong actually...")
'
expect 'Љ а ߷ ߬a ߦ' 'print("Љ а ߷ ߬a ߦ")'


describe 'Escape Sequences in String Literals'

expect $'\t' 'print("\t")'
expect $' \n' 'print(" \n")'
expect $'\t\t' 'print("\t\t")'
expect '"' 'print("\"")'
expect 'hello "world"' 'print("hello \"world\"")'
expect "'" "print(\"'\")"
expect_err 'SyntaxError' 'print("\0")'
expect_err 'SyntaxError' 'print("\9")'
expect_err 'SyntaxError' 'print("\x")'


describe 'fn Str.str'

expect '' 'print("".str())'
expect 'hi' 'print("hi".str())'


describe 'fn Str.len'

expect_expr_int '0' '"".len()'
expect_expr_int '2' '"hi".len()'
expect_expr_int '3' '"abc".len()'
expect_expr_int '4' '"abcd".len()'
expect_err 'TypeError' 'print("abcd".len())'


describe 'fn Str.at'

expect 'c'  'print("abc".at(-1).str())'
expect 'b'  'print("abc".at(-2).str())'
expect 'a'  'print("abc".at(-3).str())'
expect ''   'print("abc".at(-4).str())'
expect 'a'  'print("abc".at(0).str())'
expect 'b'  'print("abc".at(1).str())'
expect 'c'  'print("abc".at(2).str())'
expect ''   'print("abc".at(4).str())'
expect '💖' 'print("💖💖".at(0).str())'
expect ''   'print("💖💖".at(4).str())'
expect '🏳' 'print("🏳️‍🌈".at(0).str())'
# (0-width character, is something there...)
expect '️'   'print("🏳️‍🌈".at(1).str())'
expect '🇦' 'print("🇨🇦".at(1).str())'
expect '🇨' 'print("🇨🇦".at(0).str())'



describe 'fn Str.=='

expect_expr_bool 'true'  '"abc" == "abc"'
expect_expr_bool 'true'  '"💖" == "💖"'
expect_expr_bool 'true'  '"" == ""'
expect_expr_bool 'true'  '"﷽ is the longest char\n" == "﷽ is the longest char\n"'
expect_expr_bool 'false' '"abc" == "def"'
expect_expr_bool 'false' '"abc" == "abcd"'
expect_expr_bool 'false' '"abcd" == "abc"'
expect_expr_bool 'false' '"abc" == "ABC"'
expect_expr_bool 'false' '"ABC" == "abc"'
expect_expr_bool 'false' '"abc" == "ab"'
expect_expr_bool 'false' '"ab" == "abc"'
expect_expr_bool 'false' '"abc" == "abd"'
expect_expr_bool 'false' '"abd" == "abc"'
expect_expr_bool 'false' '" " == ""'
expect_expr_bool 'false' '"\n" == ""'
expect_expr_bool 'false' '"\"" == "\\\""'

describe 'fn Str.!='

expect_expr_bool 'false' '"abc" != "abc"'
expect_expr_bool 'false' '"💖" != "💖"'
expect_expr_bool 'false' '"" != ""'
expect_expr_bool 'false' '"﷽ is the longest char\n" != "﷽ is the longest char\n"'
expect_expr_bool 'true'  '"abc" != "def"'
expect_expr_bool 'true'  '"abc" != "abcd"'
expect_expr_bool 'true'  '"abcd" != "abc"'
expect_expr_bool 'true'  '"abc" != "ABC"'
expect_expr_bool 'true'  '"ABC" != "abc"'
expect_expr_bool 'true'  '"abc" != "ab"'
expect_expr_bool 'true'  '"ab" != "abc"'
expect_expr_bool 'true'  '"abc" != "abd"'
expect_expr_bool 'true'  '"abd" != "abc"'
expect_expr_bool 'true'  '" " != ""'
expect_expr_bool 'true'  '"\n" != ""'
expect_expr_bool 'true'  '"\"" != "\\\""'


describe 'fn Str.+'

expect 'abc' 'print("" + "abc")'
expect 'abc' 'print("a" + "bc")'
expect 'abc' 'print("ab" + "c")'
expect 'abc' 'print("abc" + "")'
expect 'abc' 'print("a" + "b" + "c")'
expect 'abc' 'print("a" + "b" + "c" + "")'
expect 'abc' 'print("" + "a" + "b" + "c")'
expect 'abc' 'print("" + "a" + "b" + "c" + "")'
expect '💖 ﷽' 'print("💖" + " " + "﷽")'


describe 'fn Str.concat'

expect 'abc' 'print("".concat("abc"))'
expect 'abc' 'print("a".concat("bc"))'
expect 'abc' 'print("ab".concat("c"))'
expect 'abc' 'print("abc".concat(""))'
expect 'abc' 'print("a".concat("b").concat("c"))'
expect 'abc' 'print("a".concat("b").concat("c").concat(""))'
expect '💖﷽' 'print("💖".concat("﷽"))'


describe 'fn Str.repeat'

expect '' 'print("".repeat(0))'
expect '' 'print("".repeat(1))'
expect '' 'print("".repeat(2))'
expect 'a' 'print("a".repeat(1))'
expect 'aa' 'print("a".repeat(2))'
expect 'aaa' 'print("a".repeat(3))'
expect '💖💖💖' 'print("💖".repeat(3))'
expect '' 'print("💖".repeat(0))'
expect '' 'print("💖".repeat(-1))'
