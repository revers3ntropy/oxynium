describe 'primitive Ptr'

expect '1,hi,2,3' '
    print(Ptr.make!<Int>(1).unwrap().Str());
    print(",");
    print(Ptr.make!<Str>("hi").unwrap());
    print(",");
    print(Ptr.make!<Str>("hi").unwrap().len().Str());
    print(",");
    print(Ptr.make!<Ptr<Int>>(
        Ptr.make!<Int>(3)
    ).unwrap().unwrap().Str());
'
