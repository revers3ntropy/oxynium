describe 'class Any'

expect 'Hello,World,1,0,1,true,false,true' '

    class C;

    def main () {
        let a1 = Any.from!<Str>("Hello");
        print(Any.cast!<Any, Str>(a1));
        print(",");
        let a2 = Any.cast!<Str, Any>("World");
        print(Any.cast!<Any, Str>(a2));
        print(",");
        print(Any.cast!<Bool, Int>(true).Str());
        print(",");
        print(Any.cast!<Bool, Int>(false).Str());
        print(",");
        print(Any.from!<Bool>(true).to!<Int>().Str());
        print(",");
        print((Any.cast!<Bool, Int>(true) == 1).Str());

        let c1 = new C;
        let c2 = new C;
        print(",");
        print((Any.cast!<C, Int>(c1) == Any.cast!<C, Int>(c2)).Str());
        print(",");
        print((Any.cast!<C, Int>(c1) == Any.cast!<C, Int>(c1)).Str());
    }
'


describe 'def Any.cast'

expect '1,true,true,true,true,false,A,0,0,103' '
    def void () { 2 }

    def main () {
        print(Any.cast!<Bool, Int>(true).Str());
        print(",");
        print(Any.cast!<Int, Bool>(1).Str());
        print(",");
        print(Any.cast!<Int, Bool>(2).Str());
        print(",");
        print(Any.cast!<Int, Bool>(64).Str());
        print(",");
        print(Any.cast!<Int, Bool>(-1).Str());
        print(",");
        print(Any.cast!<Int, Bool>(0).Str());
        print(",");
        print(Any.cast!<Int, Char>(65).Str());
        print(",");
        print(Any.cast!<Void, Int>(new Void).Str());
        print(",");
        84;
        print(Any.cast!<Void, Int>(void()).Str());
        print(",");
        print(Any.cast!<Char, Int>("g".at(0)).Str());
    }
'