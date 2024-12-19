describe 'class Range, def range'

expect 'Range,Fn range(start: Int, end: Int, step: Int) Range,Range,Range(0, 2, 1),3,5,6,1,3,9,2,8,1,6,2,3,0,4,1,4,0,1,0,1,2' '
def main() {
    print(typeof Range, ",")
    print(typeof range, ",")
    print(typeof range(2), ",")
    print(range(2).Str(), ",")

    print(range(3, 5, 6).start.Str(), ",")
    print(range(3, 5, 6).end.Str(), ",")
    print(range(3, 5, 6).step.Str(), ",")
    print(range(3, 5, 6).len().Str(), ",")
    print(range(3, 5, 6).at_raw(0).Str(), ",")
    // should give 9 despite that being above end,
    // as it is an unchecked access
    print(range(3, 5, 6).at_raw(1).Str(), ",")

    print(range(2, 8).start.Str(), ",")
    print(range(2, 8).end.Str(), ",")
    print(range(2, 8).step.Str(), ",")
    print(range(2, 8).len().Str(), ",")
    print(range(2, 8).at_raw(0).Str(), ",")
    print(range(2, 8).at_raw(1).Str(), ",")

    print(range(4).start.Str(), ",")
    print(range(4).end.Str(), ",")
    print(range(4).step.Str(), ",")
    print(range(4).len().Str(), ",")
    print(range(4).at_raw(0).Str(), ",")
    print(range(4).at_raw(1).Str(), ",")

    print(range(0).len().Str(), ",")
    print(range(1).len().Str(), ",")
    print(range(2).len().Str())
}
'

expect_err 'SyntaxError' 'range 1'
expect_err 'SyntaxError' 'range 1, 3'
expect_err 'SyntaxError' 'range 1..2'
expect_err 'SyntaxError' '1..2'
expect_err 'SyntaxError' '1->2'
expect_err 'SyntaxError' '1 to 2'
expect_err 'SyntaxError' 'Range 1'
expect_err 'SyntaxError' 'Range { 1 }'
expect_err 'UnknownSymbol' 'Range(1)'
expect_err 'UnknownSymbol' 'Range(1, 2)'
expect_err 'UnknownSymbol' 'Range()'