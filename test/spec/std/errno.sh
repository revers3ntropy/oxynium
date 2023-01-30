describe 'func errno'

expect '0' '
    print(errno().str())
'