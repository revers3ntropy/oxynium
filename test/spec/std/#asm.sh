describe 'macro #asm'

expect 'Void' 'print(typeof #asm "")'
expect 'hi' '
    func asm(arg: Str) Str {
        return Any.cast!<Void, Str>(#asm "
            push qword [rbp + 16]
        ")
    }
    println(asm("hi"))
'

expect_err 'TypeError' '#asm(Void, "")'
expect_err 'TypeError' '#asm 1'
expect_err 'TypeError' '#asm(1)'
expect_err 'TypeError' '#asm()'
expect_err 'SyntaxError' '#asm'
expect_err 'TypeError' '#asm("", 1)'
expect_err 'TypeError' '
    func main () {
        let s = "";
        #asm(s);
    }
'
expect_err 'TypeError' 'print(typeof #asm(1, ""))'
expect_err 'TypeError' 'print(typeof #asm("", ""))'