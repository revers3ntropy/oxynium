describe 'Option'

expect 'Option<Str>,hello,hello,Option<Str>,world,false,true,true,false,Option<Option<Option<Result<Option<Int>, Str>>>>' '
    def some () Str? {
        return Option.some!<Str>("hello");
    }

    def none () Option<Str> {
        return Option.none!<Str>();
    }

    def main () {
        print(typeof some());
        print(",");
        print(some().unwrap());
        print(",");
        print(some().unwrap_or("world"));
        print(",");
        print(typeof none());
        print(",");
        print(none().unwrap_or("world"));
        print(",");
        print(none().is_some().Str());
        print(",");
        print(none().is_none().Str());
        print(",");
        print(some().is_some().Str());
        print(",");
        print(some().is_none().Str());

        let mut op_op: Result<Int?, Str>???;
        print(",");
        print(typeof op_op);
    }
'