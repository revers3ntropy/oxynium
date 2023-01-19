describe 'macro #asm'

expect 'Void' 'print(typeof #asm "")'
expect 'Str' 'print(typeof #asm(Str, ""))'
expect 'hi' '
    fn asm(arg: Str) Str {
        return #asm(Str, "
            push qword [rbp + 16]
        ")
    }
    println(asm("hi"))
'

expect_err 'TypeError' '#asm 1'
expect_err 'TypeError' '#asm(1)'
expect_err 'TypeError' '#asm()'
expect_err 'SyntaxError' '#asm'
expect_err 'TypeError' '#asm("", 1)'
expect_err 'TypeError' '
    fn main () {
        let s = "";
        #asm(s);
    }
'
expect_err 'TypeError' 'print(typeof #asm(1, ""))'
expect_err 'TypeError' 'print(typeof #asm("", ""))'