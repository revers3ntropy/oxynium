describe 'macro #unchecked_cast'

expect 'Void' 'print(typeof #unchecked_cast Void "")'
expect 'Int' 'print(typeof #unchecked_cast Int, "")'
expect 'Result<Str, Void>' '
    print(typeof #unchecked_cast(Result<Str, Void>, Option.some!<Int>(1)))
'
expect 'Result<Str, Option<Int>>' '
    print(typeof #unchecked_cast Result<Str, Option<Int>> 2)
'
expect 'false' '
    def unchecked_cast() Bool {
        return #unchecked_cast Bool 0
    }
    print(unchecked_cast().Str())
'

expect_err 'SyntaxError' '#unchecked_cast 1'
expect_err 'SyntaxError' '#unchecked_cast Int'
expect_err 'SyntaxError' '#unchecked_cast(1)'
expect_err 'SyntaxError' '#unchecked_cast()'
expect_err 'SyntaxError' '#unchecked_cast'
expect_err 'SyntaxError' '#unchecked_cast Void, "")'
expect_err 'SyntaxError' '#unchecked_cast("", 1)'
expect_err 'SyntaxError' '#unchecked_cast(2, 1)'
expect_err 'UnknownSymbol' '#unchecked_cast(Fish, 1)'
expect_err 'SyntaxError' '
    def main () {
        let s = "";
        #unchecked_cast(s);
    }
'