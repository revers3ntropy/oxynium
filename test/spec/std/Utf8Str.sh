describe 'Utf8Str'

expect 'hi,ho,2' '
    println(Any.cast!<Utf8Str, Str>("hi".Utf8Str()));
    print(",");
    println("ho".Utf8Str().Str());
    print(",");
    println("ho".Utf8Str().Str().len().Str());
'