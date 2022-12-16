describe 'Struct Declarations'

expect '' 'struct S {};'
expect '' '
    struct S {};
    fn do_nothing(s: S) {};
'
expect '' '
    struct S {
        x: Int;
        y: Int;
    };
'


describe 'Struct Instantiation'

expect '' '
    struct S {
        x: Int;
        y: Int;
    };
    new S { x: 1, y: 2 };
'