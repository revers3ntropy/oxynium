describe '#asm'

expect_err 'TypeError' '#asm 1'
expect_err 'TypeError' '#asm(1)'
expect_err 'TypeError' '#asm()'
expect_err 'SyntaxError' '#asm'
expect_err 'TypeError' '#asm("", 1)'
expect_err 'TypeError' '#asm("", "")'
expect_err 'TypeError' '
    fn main () {
        let s = "";
        #asm(s);
    }
'

expect 'Void' 'print(typeof #asm "")'