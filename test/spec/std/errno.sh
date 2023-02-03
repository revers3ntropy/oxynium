describe 'def errno'

expect '0' '
    print(errno().Str())
'