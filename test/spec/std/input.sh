describe 'def input'

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
expect     'true'  'print((input() == "a").Str())'      'a'
expect     'true'  '
    def main () {
        let i = input();
        print((i == "a").Str())
    }
' 'a'


describe 'def input Giving Correctly Encoded Str'

expect     'true'  'print((input() == "abcdef").Str())' 'abcdef'
expect     'false' 'print((input() == "a").Str())'      'b'
expect     'false' 'print((input() == "abc").Str())'    'bca'
expect     'false' 'print((input() == "a").Str())'      'ab'
expect     'false' 'print((input() == "ab").Str())'     'a'
expect     'a' $'print(input().at(0).or(\' \').Str())' 'a'
expect     'a' $'print(input().at(0).or(\' \').Str())' 'abc'
expect     'a' $'print(input().at(0).or(\' \').Str())' 'aabc'
expect     '􏿽' $'print(input().at(2).or(\' \').Str())' '12􏿽45'
expect     '4' $'print(input().at(3).or(\' \').Str())' '12􏿽45'
expect     '2' $'print(input().at(1).or(\' \').Str())' '12􏿽45'
expect     '🏳' $'print(input().at(0).or(\' \').Str())' '🏳️‍🌈'
# 0-width character (there is something there...)
expect     '️' $'print(input().at(1).or(\' \').Str())' '🏳️‍🌈'
expect     '🇦' $'print(input().at(1).or(\' \').Str())' '🇨🇦'
expect     '🇨' $'print(input().at(0).or(\' \').Str())' '🇨🇦'
