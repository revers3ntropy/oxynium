describe 'Utf8Str'

expect 'hi,ho,2' '
    print(#unchecked_cast Str "hi");
    print(",");
    print("ho".Utf8Str().Str());
    print(",");
    print("ho".Utf8Str().Str().len().Str());
'