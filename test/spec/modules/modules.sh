describe 'Modules'

expect 'public' '
    #import "my_module.oxy"
    my_module.do_something_public()
'

expect_err 'TypeError' '
    #import "my_module.oxy"
    my_module.do_something_private()
'