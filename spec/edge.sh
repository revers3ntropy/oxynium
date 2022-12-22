describe 'Edge Cases'

expect '' ''
expect '' ' '
expect '' '


 '

expect_err 'SyntaxError' ';'
expect_err 'SyntaxError' ';;'
expect ''  ';1'
expect ''  ';1;'
expect ''  ';;;;1;;;;;'
expect ''  '1;;;;;;'
expect ''  '1;'
expect ''  '1'


expect_err 'SyntaxError' '#'
expect_err 'SyntaxError' '*&^%$#@!~'

# weird EOF cases
expect_err 'UnknownSymbol' 'mut'
expect_err 'SyntaxError' 'const'
expect_err 'SyntaxError' 'let'
expect_err 'SyntaxError' 'let mut'
expect_err 'SyntaxError' 'fn'
expect_err 'SyntaxError' 'extern'
expect_err 'SyntaxError' 'extern fn'
expect_err 'SyntaxError' 'extern const a'
expect_err 'SyntaxError' 'extern const a:'
expect_err 'SyntaxError' 'extern const a: Str = '
expect_err 'SyntaxError' 'const a: Str = '
expect_err 'SyntaxError' 'fn a'
expect_err 'SyntaxError' 'fn a('
expect_err 'SyntaxError' 'fn a(a'
expect_err 'SyntaxError' 'fn a(a: '
expect_err 'SyntaxError' 'fn a(a: Str'
expect_err 'SyntaxError' 'fn a(a: Str, b'
expect_err 'SyntaxError' 'fn a(a: Str, b:'
expect_err 'SyntaxError' 'fn a(a: Str, b: a'
expect_err 'SyntaxError' 'fn a(a: Str) {'
expect_err 'SyntaxError' 'fn a(a: Str) { 1;'
expect_err 'SyntaxError' 'fn a(a: Str,'
expect_err 'SyntaxError' 'fn a(a: Str, b'
expect_err 'SyntaxError' 'fn a(a: Str, b:'
expect_err 'SyntaxError' 'fn a(a: Str, b: a'
expect_err 'SyntaxError' 'fn a(a: Str, b: a,'
expect_err 'SyntaxError' 'fn a(a: Str, b: a, c'
expect_err 'SyntaxError' 'fn a(a: Str, b, c'
expect_err 'SyntaxError' 'while'
expect_err 'SyntaxError' 'while {'
expect_err 'SyntaxError' 'while { 1;'
expect_err 'SyntaxError' 'while true'
expect_err 'SyntaxError' 'while true {'
expect_err 'SyntaxError' 'while true { 1;'
expect_err 'SyntaxError' 'if'
expect_err 'SyntaxError' 'if {'
expect_err 'SyntaxError' 'if {}'
expect_err 'SyntaxError' 'if true'
expect_err 'SyntaxError' 'if true {'
expect_err 'SyntaxError' 'if true { 1;'
expect_err 'SyntaxError' 'class'
expect_err 'SyntaxError' 'class S {'
expect_err 'SyntaxError' 'class {}'
expect_err 'SyntaxError' 'class S { x'
expect_err 'SyntaxError' 'class S { x:'
expect_err 'SyntaxError' 'class S { x: Str'
expect_err 'SyntaxError' 'class S { fn'
expect_err 'SyntaxError' 'class S { fn a'
expect_err 'SyntaxError' 'class S { fn a('
expect_err 'SyntaxError' 'class S { fn f(self'
expect_err 'SyntaxError' 'class S { fn f(self:'
expect_err 'SyntaxError' 'class S { fn f(self,'
expect_err 'SyntaxError' 'class S { fn a(self, a'
expect_err 'SyntaxError' 'class S { fn a(self, a: '
expect_err 'SyntaxError' 'class S { fn a(self, a: Str'
expect_err 'SyntaxError' 'class S { fn a(self, a: Str) {'
expect_err 'SyntaxError' 'class S { fn a(self, a: Str) { 1;'
expect_err 'SyntaxError' 'class S { fn a(self, a: Str) { 1; }'
expect_err 'SyntaxError' 'class S { fn f(self: ) {} }'
expect_err 'SyntaxError' 'new '
expect_err 'UnknownSymbol' 'new S'
expect_err 'SyntaxError' 'new S { '
expect_err 'SyntaxError' 'new S { a'
expect_err 'SyntaxError' 'new S { a:'
expect_err 'SyntaxError' 'new S { a:'
expect_err 'SyntaxError' 'new S { fn'
expect_err 'SyntaxError' 'new S { fn s'

expect_err 'SyntaxError' '*'
expect_err 'SyntaxError' '/'
expect_err 'SyntaxError' '+'
expect_err 'SyntaxError' '=='
expect_err 'SyntaxError' '1+'
expect_err 'SyntaxError' '1 1'
expect_err 'SyntaxError' '1 1-'
expect_err 'SyntaxError' '1 -1-'
expect_err 'SyntaxError' '1 -1-'
expect_err 'SyntaxError' '1 */ 2'
expect_err 'SyntaxError' '1 * - / 2'
expect_err 'SyntaxError' '1 = 2'
expect_err 'SyntaxError' '1 + 2 = 3'
expect_err 'SyntaxError' '*1'
expect_err 'SyntaxError' '+1'
expect_err 'SyntaxError' '+-2'
expect_err 'SyntaxError' '('
expect_err 'SyntaxError' '()'
expect_err 'SyntaxError' '(1()'
expect_err 'SyntaxError' '(2+2'
expect_err 'SyntaxError' '((2+2)'
expect_err 'SyntaxError' '((2+(2*6))'
expect_err 'SyntaxError' '((2+(2*6)'