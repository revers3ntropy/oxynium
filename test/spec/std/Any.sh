describe 'class Any'

expect 'Hello,World,false,false,true,1,0,1,true,false,true' '

    class C;

    fn main () {
        let a1 = Any.from!<Str>("Hello");
        print(Any.cast!<Any, Str>(a1));
        print(",");
        let a2 = Any.cast!<Str, Any>("World");
        print(Any.cast!<Any, Str>(a2));
        print(",");
        print(a1.eq!<Any>(a2).str());
        print(",");
        print(a1.eq!<Str>("Hello").str());
        print(",");
        print(a1.eq!<Any>(a1).str());
        print(",");
        print(Any.from!<Bool>(true).str());
        print(",");
        print(Any.from!<Bool>(false).str());
        print(",");
        print(Any.from!<Bool>(true).to!<Int>().str());
        print(",");
        print(Any.from!<Bool>(true).eq!<Bool>(true).str());

        let c1 = new C;
        let c2 = new C;
        print(",");
        print(Any.from!<C>(c1).eq!<C>(c2).str());
        print(",");
        print(Any.from!<C>(c1).eq!<C>(c1).str());
    }
'


describe 'fn Any.cast'

expect '1,true,true,true,true,false,A,0,0,103' '
    fn void () { 2 }

    fn main () {
        print(Any.cast!<Bool, Int>(true).str());
        print(",");
        print(Any.cast!<Int, Bool>(1).str());
        print(",");
        print(Any.cast!<Int, Bool>(2).str());
        print(",");
        print(Any.cast!<Int, Bool>(64).str());
        print(",");
        print(Any.cast!<Int, Bool>(-1).str());
        print(",");
        print(Any.cast!<Int, Bool>(0).str());
        print(",");
        print(Any.cast!<Int, Char>(65).str());
        print(",");
        print(Any.cast!<Void, Int>(new Void).str());
        print(",");
        84;
        print(Any.cast!<Void, Int>(void()).str());
        print(",");
        print(Any.cast!<Char, Int>("g".at(0)).str());
    }
'