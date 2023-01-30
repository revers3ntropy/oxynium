describe 'class Result'

expect '1,2,true,false' '
    func main () {
        print(Result.ok!<Int, Int>(1).unwrap().str());
        print(",");
        print(Result.ok!<Int, Str>(2).unwrap().str());
        print(",");
        print(Result.ok!<Int, Str>(2).is_ok().str());
        print(",");
        print(Result.ok!<Int, Str>(2).is_err().str());
    }
'
expect '1,hi,false,true' '
    func main () {
        print(Result.err!<Int, Int>(1).error.unwrap().str());
        print(",");
        print(Result.err!<Int, Str>("hi").error.unwrap().str());
        print(",");
        print(Result.err!<Int, Str>("").is_ok().str());
        print(",");
        print(Result.err!<Int, Str>("").is_err().str());
    }
'