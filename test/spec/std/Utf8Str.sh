describe 'Utf8Str'

expect 'hi,ho,2' '
    print(Any.cast!<Utf8Str, Str>("hi".Utf8Str()));
    print(",");
    print("ho".Utf8Str().Str());
    print(",");
    print("ho".Utf8Str().Str().len().Str());
'