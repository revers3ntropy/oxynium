describe 'List'

expect '1 Hi' '
    fn main () {
        let l = new List<Int> {
            start: 1,
            size: 1
        };
        print(l.at(0).str());

        let l2 = new List<Str> {
            start: " Hi",
            size: 1
        };
        print(l2.at(0));
    }
'