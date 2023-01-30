describe 'primitive Ptr'

expect '1,hi,2,3' '
    print(Ptr.make!<Int>(1).unwrap().str());
    print(",");
    print(Ptr.make!<Str>("hi").unwrap());
    print(",");
    print(Ptr.make!<Str>("hi").unwrap().len().str());
    print(",");
    print(Ptr.make!<Ptr<Int>>(
        Ptr.make!<Int>(3)
    ).unwrap().unwrap().str());
'


describe 'Ptr.allocate'

expect "PANIC: 'cannot allocate a block of memory of size 0 or less'" '
    Ptr.allocate!<Int>(0)
'
expect "PANIC: 'cannot allocate a block of memory of size 0 or less'" '
    Ptr.allocate!<Int>(-99)
'
expect '0' '
    func main () {
        print(Ptr.allocate!<Int>(1).unwrap().str())
    }
'