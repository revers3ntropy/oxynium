describe 'macro #asm'

expect 'Void' 'print(typeof #asm "")'
expect 'hi' '
    def asm(arg: Str) Str {
        return Any.cast!<Void, Str>(#asm "
            push qword [rbp + 16]
        ")
    }
    print(asm("hi"))
'

expect_err 'TypeError' '#asm(Void, "")'
expect_err 'TypeError' '#asm 1'
expect_err 'TypeError' '#asm(1)'
expect_err 'TypeError' '#asm()'
expect_err 'SyntaxError' '#asm'
expect_err 'TypeError' '#asm("", 1)'
expect_err 'TypeError' '
    def main () {
        let s = "";
        #asm(s);
    }
'
expect_err 'TypeError' 'print(typeof #asm(1, ""))'
expect_err 'TypeError' 'print(typeof #asm("", ""))'