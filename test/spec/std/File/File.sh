describe 'class File'

#true,false,Hello World!,Hello World!,Hello World!,Hello World!
expect 'true,false' '
    func main () {
        let path = "./test/spec/std/File/example1.txt";
        let mut f = File.open(path)
                        .unwrap("Failed to open file");
        print(f.is_open().str());
        print(",");
        f.close();
        print(f.is_open().str());
//        print(",");
//
//        f = File.open("./" + path);
//        f.write("Hello World!");
//        f.close();
//
//        f = File.open(path);
//        print(f.read_to_str());
//        print(",");
//        // make sure nothing changed
//        print(f.read_to_str());
//        f.close();
//
//        // and again
//        f = File.open("./" + path);
//        print(f.read_to_str());
//        print(",");
//        print(f.read_to_str());
//        f.close();
    }
'