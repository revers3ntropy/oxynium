describe 'class Str'

expect '' 'print(new Str.Str().Str())'
expect '' 'print(new Str.Str())'
expect '' 'print(new Str)'
expect 'Hi' 'print("Hi")'
expect_err 'SyntaxError' 'print("hi'
expect_err 'SyntaxError' '"hi'

expect 'hi' '
    def main () {
        class Str;
        print("hi");
    }
'


describe 'class Str UTF-8 Support'

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

expect           $'\t'           'print("\t")'
expect           $' \n'          'print(" \n")'
expect           $'\t\t'         'print("\t\t")'
expect           '"'             'print("\"")'
expect           'hello "world"' 'print("hello \"world\"")'
expect           "'"             "print(\"'\")"
expect_expr_bool 'true'          '"a\
b" == "ab"'
expect_err       'SyntaxError'   'print("\0")'
expect_err       'SyntaxError'   'print("\9")'
expect_err       'SyntaxError'   'print("\x")'


describe 'def Str.Str'

expect ''   'print("".Str())'
expect 'hi' 'print("hi".Str())'


describe 'def Str.len'

expect_expr_int '0'    '"".len()'
expect_expr_int '2'    '"hi".len()'
expect_expr_int '3'    '"abc".len()'
expect_expr_int '4'    '"abcd".len()'
expect_err 'TypeError' 'print("abcd".len())'


describe 'def Str.at'

expect 'c'  'print("abc".at(-1).Str())'
expect 'b'  'print("abc".at(-2).Str())'
expect 'a'  'print("abc".at(-3).Str())'
expect ''   'print("abc".at(-4).Str())'
expect 'a'  'print("abc".at(0).Str())'
expect 'b'  'print("abc".at(1).Str())'
expect 'c'  'print("abc".at(2).Str())'
expect ''   'print("abc".at(4).Str())'
expect '💖' 'print("💖💖".at(0).Str())'
expect ''   'print("💖💖".at(4).Str())'
expect '🏳' 'print("🏳️‍🌈".at(0).Str())'
# (0-width character, is something there...)
expect '️'  'print("🏳️‍🌈".at(1).Str())'
expect '🇦' 'print("🇨🇦".at(1).Str())'
expect '🇨' 'print("🇨🇦".at(0).Str())'



describe 'def Str.=='

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

describe 'def Str.!='

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


describe 'def Str.+'

expect 'abc'  'print("" + "abc")'
expect 'abc'  'print("a" + "bc")'
expect 'abc'  'print("ab" + "c")'
expect 'abc'  'print("abc" + "")'
expect 'abc'  'print("a" + "b" + "c")'
expect 'abc'  'print("a" + "b" + "c" + "")'
expect 'abc'  'print("" + "a" + "b" + "c")'
expect 'abc'  'print("" + "a" + "b" + "c" + "")'
expect '💖 ﷽' 'print("💖" + " " + "﷽")'


describe 'def Str.concat'

expect 'abc' 'print("".concat("abc"))'
expect 'abc' 'print("a".concat("bc"))'
expect 'abc' 'print("ab".concat("c"))'
expect 'abc' 'print("abc".concat(""))'
expect 'abc' 'print("a".concat("b").concat("c"))'
expect 'abc' 'print("a".concat("b").concat("c").concat(""))'
expect '💖﷽' 'print("💖".concat("﷽"))'


describe 'def Str.repeat'

expect ''       'print("".repeat(0))'
expect ''       'print("".repeat(1))'
expect ''       'print("".repeat(2))'
expect 'a'      'print("a".repeat(1))'
expect 'aa'     'print("a".repeat(2))'
expect 'aaa'    'print("a".repeat(3))'
expect '💖💖💖' 'print("💖".repeat(3))'
expect ''       'print("💖".repeat(0))'
expect ''       'print("💖".repeat(-1))'


describe 'def Str.find'

expect_expr_int '0'  '"abc".find("a")'
expect_expr_int '1'  '"abc".find("b")'
expect_expr_int '2'  '"abc".find("c")'
expect_expr_int '0'  '"abc".find("ab")'
expect_expr_int '1'  '"abc".find("bc")'
expect_expr_int '0'  '"abc".find("abc")'
expect_expr_int '-1' '"abc".find("d")'
expect_expr_int '-1' '"abc".find("abz")'
expect_expr_int '0'  '"abc".find("")'
expect_expr_int '0'  '"".find("")'
expect_expr_int '-1' '"".find("a")'
expect_expr_int '-1' '"".find("ab")'
expect_expr_int '1'  '"a💖b💖c💖".find("💖")'
expect_expr_int '3'  '"a💖b💖c💖".find("💖c")'
expect_expr_int '3'  '"a💖b💖c💖".find("💖c💖")'


describe 'def Str.contains'

expect_expr_bool 'true'  '"abc".contains("a")'
expect_expr_bool 'true'  '"abc".contains("b")'
expect_expr_bool 'true'  '"abc".contains("c")'
expect_expr_bool 'true'  '"abc".contains("ab")'
expect_expr_bool 'true'  '"abc".contains("bc")'
expect_expr_bool 'true'  '"abc".contains("abc")'
expect_expr_bool 'false' '"abc".contains("d")'
expect_expr_bool 'false' '"abc".contains("abz")'
expect_expr_bool 'true'  '"abc".contains("")'
expect_expr_bool 'true'  '"".contains("")'
expect_expr_bool 'false' '"".contains("a")'
expect_expr_bool 'false' '"".contains("ab")'
expect_expr_bool 'true'  '"a💖b💖c💖".contains("💖")'
expect_expr_bool 'true'  '"a💖b💖c💖".contains("💖c")'
expect_expr_bool 'true'  '"a💖b💖c💖".contains("💖c💖")'


describe 'def Str.utf8_size'

expect_expr_int '0'  '"".utf8_size()'
expect_expr_int '1'  '"a".utf8_size()'
expect_expr_int '3'  '"abc".utf8_size()'
expect_expr_int '4'  '"💖".utf8_size()'
expect_expr_int '3'  '"﷽".utf8_size()'
expect_expr_int '7'  '"💖﷽".utf8_size()'
expect_expr_int '8'  '"🇨🇦".utf8_size()'
expect_expr_int '19' '"1🇨🇦2💖3﷽4".utf8_size()'


describe 'def Str.substr'

expect '' 'print("".substr(0, 0))'
expect '' 'print("".substr(0, 1))'
expect '' 'print("".substr(0, 2))'
expect '' 'print("".substr(1, 0))'
expect '' 'print("".substr(1, 1))'
expect '' 'print("".substr(-1, 0))'

expect 'a'   'print("abc".substr(0, 1))'
expect ''    'print("abc".substr(0, 0))'
expect ''    'print("abc".substr(1, 1))'
expect 'b'   'print("abc".substr(1, 2))'
expect 'bc'  'print("abc".substr(1, 3))'
expect 'bc'  'print("abc".substr(1, 4))'
expect 'bc'  'print("abc".substr(1, 5))'
expect 'abc' 'print("abc".substr(0, 6))'
expect ''    'print("abc".substr(5, 7))'
expect ''    'print("abc".substr(3, 2))'
expect ''    'print("abc".substr(3, 0))'
expect 'abc' 'print("abc".substr(0))'
expect 'bc'  'print("abc".substr(1))'
expect 'c'   'print("abc".substr(2))'
expect ''    'print("abc".substr(3))'
expect ''    'print("abc".substr(2, 0))'
expect ''    'print("abc".substr(2, 1))'
expect 'the lazy dog. | quick brown fox | dog. | lazy' '
    // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/slice
    const s = "The quick brown fox jumps over the lazy dog.";
    print(s.substr(31));
    print(" | ");
    print(s.substr(4, 19));
    print(" | ");
    print(s.substr(-4));
    print(" | ");
    print(s.substr(-9, -5));
'


describe 'def Str.starts_with'

expect_expr_bool 'true'  '"a".starts_with("a")'
expect_expr_bool 'true'  '"abc".starts_with("")'
expect_expr_bool 'true'  '"abc".starts_with("a")'
expect_expr_bool 'true'  '"abc".starts_with("ab")'
expect_expr_bool 'true'  '"abc".starts_with("abc")'
expect_expr_bool 'false' '"abc".starts_with("b")'
expect_expr_bool 'false' '"abc".starts_with("bc")'
expect_expr_bool 'false' '"abc".starts_with("d")'
expect_expr_bool 'false' '"abc".starts_with("abz")'
expect_expr_bool 'false' '"abc".starts_with("abcz")'
expect_expr_bool 'false' '"abc".starts_with("abcc")'
expect_expr_bool 'false' '"".starts_with("a")'
expect_expr_bool 'false' '"".starts_with("ab")'
expect_expr_bool 'true'  '"".starts_with("")'
expect_expr_bool 'true'  '"a💖b💖c💖".starts_with("a")'
expect_expr_bool 'true'  '"a💖b💖c💖".starts_with("a💖")'
expect_expr_bool 'true'  '"a💖b💖c💖".starts_with("a💖b💖")'
expect_expr_bool 'false' '"a💖b💖c💖".starts_with("💖")'


describe 'def Str.ends_with'

expect_expr_bool 'true'  '"a".ends_with("a")'
expect_expr_bool 'true'  '"abc".ends_with("")'
expect_expr_bool 'true'  '"abc".ends_with("c")'
expect_expr_bool 'true'  '"abc".ends_with("bc")'
expect_expr_bool 'true'  '"abc".ends_with("abc")'
expect_expr_bool 'false' '"abc".ends_with("b")'
expect_expr_bool 'false' '"abc".ends_with("ab")'
expect_expr_bool 'false' '"abc".ends_with("d")'
expect_expr_bool 'false' '"abc".ends_with("zbc")'
expect_expr_bool 'false' '"abc".ends_with("zabc")'
expect_expr_bool 'false' '"abc".ends_with("cb")'
expect_expr_bool 'false' '"".ends_with("a")'
expect_expr_bool 'false' '"".ends_with("ab")'
expect_expr_bool 'true'  '"".ends_with("")'
expect_expr_bool 'true'  '"a💖b💖c💖".ends_with("💖")'
expect_expr_bool 'true'  '"a💖b💖c💖".ends_with("c💖")'
expect_expr_bool 'true'  '"a💖b💖c💖".ends_with("b💖c💖")'
expect_expr_bool 'true'  '"a💖b💖c💖".ends_with("a💖b💖c💖")'
expect_expr_bool 'false' '"a💖b💖c💖".ends_with("a💖b💖c💖💖")'
expect_expr_bool 'false' '"a💖b💖c💖".ends_with("a💖b💖c💖d")'


describe 'def Str.reversed'

expect ''           'print("".reversed())'
expect 'a'          'print("a".reversed())'
expect 'cba'        'print("abc".reversed())'
expect '💖'         'print("💖".reversed())'
expect '4💖3💖2💖1' 'print("1💖2💖3💖4".reversed())'


describe 'def Str.replace'

expect 'b'       'print("a".replace("a", "b"))'
expect 'bbb'     'print("aaa".replace("a", "b"))'
expect 'bb'      'print("ab".replace("a", "b"))'
expect 'ab'      'print("ab".replace("", "b"))'
expect 'ab'      'print("a".replace("a", "ab"))'
expect '💖'      'print("a".replace("a", "💖"))'
expect '💖b'     'print("ab".replace("a", "💖"))'
expect 'ab'      'print("ab".replace("", "💖"))'
expect 'ab'      'print("ab".replace(""))'
expect ''        'print("".replace(""))'
expect ''        'print("".replace("abc", "def"))'
expect ''        'print("".replace("a", "b"))'
expect 'bc'      'print("abc".replace("a"))'
expect 'c'       'print("abc".replace("ab"))'
expect 'c'       'print("abc".replace("ab"))'
expect '💖'      'print("abc".replace("abc", "💖"))'
expect 'ba'      'print("💖a".replace("💖", "b"))'
expect '💖💖'    'print("💖a".replace("a", "💖"))'
expect 'That at' 'print("This is".replace("is", "at"))'
expect 'That is' 'print("This is".replace("is", "at", 1))'
expect 'That at' 'print("This is".replace("is", "at", 2))'
expect 'That at' 'print("This is".replace("is", "at", 3))'
expect 'That at' 'print("This is".replace("is", "at", -1))'
expect 'That at' 'print("This is".replace("is", "at", -10))'
expect 'This is' 'print("This is".replace("is", "at", 0))'


describe 'def Str.insert'

expect 'a'       'print("".insert(0, "a"))'
expect 'a'       'print("".insert(1, "a"))'
expect 'a'       'print("".insert(-1, "a"))'
expect 'a'       'print("".insert(-10, "a"))'
expect 'a'       'print("".insert(10, "a"))'
expect 'ab'      'print("a".insert(1, "b"))'
expect 'ba'      'print("a".insert(0, "b"))'
expect '💖a'     'print("a".insert(0, "💖"))'
expect 'a💖'     'print("a".insert(1, "💖"))'
expect '💖a💖'   'print("a💖".insert(0, "💖"))'
expect '💖a💖'   'print("💖a".insert(2, "💖"))'
expect '12345'   'print("1234".insert(-1, "5"))'
expect '12345'   'print("1235".insert(-2, "4"))'
expect '12345'   'print("1245".insert(-3, "3"))'
expect '123456' 'print("123".insert(100, "456"))'


describe 'def Str.remove'

expect ''   'print("".remove(0, 0))'
expect ''   'print("".remove(0, 1))'
expect ''   'print("".remove(1, 0))'
expect ''   'print("".remove(1, 1))'
expect ''   'print("".remove(0, -1))'
expect ''   'print("".remove(-1, 0))'
expect ''   'print("".remove(-1, -1))'
expect 'a'  'print("a".remove(0, 0))'
expect ''   'print("a".remove(0, 1))'
expect 'a'  'print("a".remove(1, 0))'
expect 'a'  'print("a".remove(1, 1))'
expect 'a'  'print("a".remove(0, -1))'
expect 'a'  'print("a".remove(-1, 0))'
expect 'a'  'print("a".remove(-1, -1))'
expect 'a'  'print("ab".remove(1, 1))'
expect 'b'  'print("ab".remove(0, 1))'
expect 'ab' 'print("ab".remove(1, 0))'
expect 'ab' 'print("ab".remove(1, -1))'
expect 'a'  'print("ab".remove(-1, 1))'
expect 'b'  'print("ab".remove(-2, 1))'
expect ''   'print("ab".remove(-2, 2))'
expect 'a'  'print("abc".remove(1, 5))'
expect 'bcdef'  'print("abcdef".remove(-6, 1))'
expect 'bcdef'  'print("abcdef".remove(-7, 2))'
expect 'bcdef'  'print("abcdef".remove(-9, 4))'

describe 'def Str.Int'

expect '1,2,3,4,5,6,7,8,9,0,-1,90,-90,9,9223372036854775807,false,false,false,false' '
    print("1".Int().unwrap().Str())
    print(",")
    print(Str.Int("2").unwrap().Str())
    print(",")
    print(Str.Int("3").unwrap().Str())
    print(",")
    print(Str.Int("4").unwrap().Str())
    print(",")
    print(Str.Int("5").unwrap().Str())
    print(",")
    print(Str.Int("6").unwrap().Str())
    print(",")
    print(Str.Int("7").unwrap().Str())
    print(",")
    print(Str.Int("8").unwrap().Str())
    print(",")
    print(Str.Int("9").unwrap().Str())
    print(",")
    print(Str.Int("0").unwrap().Str())
    print(",")
    print(Str.Int("-1").unwrap().Str())
    print(",")
    print(Str.Int("90").unwrap().Str())
    print(",")
    print(Str.Int("-90").unwrap().Str())
    print(",")
    print(Str.Int("009").unwrap().Str())
    print(",")
    print(Str.Int("9223372036854775807").unwrap().Str())
    print(",")
    print(Str.Int("not a number").ok.Str())
    print(",")
    print(Str.Int("").ok.Str())
    print(",")
    print(Str.Int("0.1").ok.Str())
    print(",")
    print(Str.Int("9223372036854775808").ok.Str())
'
