describe 'fn input'

expect 'abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ' '
    print(input())
' 'abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ'
expect     'Hello, World!'     'print(input())'         'Hello, World!'
expect     'a'                 'print(input())'         'a'
expect     '£'                 'print(input())'         '£'
expect     '􏿽'                 'print(input())'         '􏿽'
expect     'Љ а ߷ ߬a ߦ'         'print(input())'         'Љ а ߷ ߬a ߦ'
expect     '1:2'               'print(input("1:"))'     '2'
expect     '1:2'               'print(input("1:", 1))'  '2ggfdgf'
expect     '1:2'               'print(input("1:", 4))'  '2'
expect_err 'TypeError'         'input(true)'            '2'
expect_err 'TypeError'         'input(4)'               '2'
expect_err 'TypeError'         'input("h", true)'       '2'
expect     'true'  'print((input() == "a").str())'      'a'
expect     'true'  '
    fn main () {
        let i = input();
        print((i == "a").str())
    }
' 'a'


describe 'fn input Giving Correctly Encoded Str'

expect     'true'  'print((input() == "abcdef").str())' 'abcdef'
expect     'false' 'print((input() == "a").str())'      'b'
expect     'false' 'print((input() == "abc").str())'    'bca'
expect     'false' 'print((input() == "a").str())'      'ab'
expect     'false' 'print((input() == "ab").str())'     'a'
expect     'a'     'print(input().at(0).str())'         'a'
expect     'a'     'print(input().at(0).str())'         'abc'
expect     'a'     'print(input().at(0).str())'         'aabc'
expect     '􏿽'     'print(input().at(2).str())'         '12􏿽45'
expect     '4'     'print(input().at(3).str())'         '12􏿽45'
expect     '2'     'print(input().at(1).str())'         '12􏿽45'
expect     '🏳'    'print(input().at(0).str())'         '🏳️‍🌈'
# (0-width character, is something there...)
expect     '️'      'print(input().at(1).str())'         '🏳️‍🌈'
expect     '🇦'    'print(input().at(1).str())'         '🇨🇦'
expect     '🇨'    'print(input().at(0).str())'         '🇨🇦'
