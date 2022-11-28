describe 'Function Declarations'

expect 'fn a()' ''
expect 'fn a(a: Int, b: Bool, c: Str)' ''
expect 'fn a(): Void' ''
expect 'fn a(a: Int): Str' ''
expect_err 'fn a(a): Str' 'SyntaxError'