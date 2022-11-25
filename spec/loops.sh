describe 'Loops'

expect '
var i = 0;
for {
    i = i + 1;
    print_int(i);
    print_nl();
    break;
}
' '1'

expect '
for {
    print("hello");
    break;
}
for {
    print("there");
    break;
}
' $'hello\rthere'