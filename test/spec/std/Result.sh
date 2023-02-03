describe 'class Result'

expect '1,2,true,false' '
    def main () {
        print(Result.ok!<Int, Int>(1).unwrap().Str());
        print(",");
        print(Result.ok!<Int, Str>(2).unwrap().Str());
        print(",");
        print(Result.ok!<Int, Str>(2).is_ok().Str());
        print(",");
        print(Result.ok!<Int, Str>(2).is_err().Str());
    }
'
expect '1,hi,false,true' '
    def main () {
        print(Result.err!<Int, Int>(1).error.unwrap().Str());
        print(",");
        print(Result.err!<Int, Str>("hi").error.unwrap().Str());
        print(",");
        print(Result.err!<Int, Str>("").is_ok().Str());
        print(",");
        print(Result.err!<Int, Str>("").is_err().Str());
    }
'