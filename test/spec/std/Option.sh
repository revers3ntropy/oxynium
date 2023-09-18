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
        print(some().or("world"));
        print(",");
        print(typeof none());
        print(",");
        print(none().or("world"));
        print(",");
        print(none().is_some.Str());
        print(",");
        print((!none().is_some).Str());
        print(",");
        print(some().is_some.Str());
        print(",");
        print((!some().is_some).Str());

        let mut op_op: Result<Int?, Str>???;
        print(",");
        print(typeof op_op);
    }
'

describe 'Option None Coalescing'

expect '5,1,0,5,1,0' '
    def main () {
        let op: Option<Int> = Option.none!<Int>();
        let op2: Int? = Option.some!<Int>(1);
        let op3 = Option.some!<Int>(0);

        print(op.or(5).Str());
        print(",");
        print(op2.or(5).Str());
        print(",");
        print(op3.or(5).Str());
        print(",");
        print((op ?? 5).Str());
        print(",");
        print((op2 ?? 5).Str());
        print(",");
        print((op3 ?? 5).Str());
    }
'


describe '`?` Type Operator uses Global `Option` Class'

expect '0,Option<C>' '
    def f () Int? {
        return Option.none!<Int>();
    }

    def main () {
        class Option;
        class C;

        let op: Int? = f();
        print((op ?? 0).Str());

        print(",");

        let mut c: C?;
        print(typeof c);
    }
'
