describe 'class File'

#true,false,Hello World!,Hello World!,Hello World!,Hello World!
expect 'true,false,Hello World!,Hello World!,Hello World!,Hello World!,' '
    def main () {
        let path = "test/spec/std/File/example1.txt";
        let mut f = File.open(path)
                        .unwrap("Failed to open file");
        print(f.is_open().Str());
        print(",");
        f.close();
        print(f.is_open().Str());
        print(",");

        f = File.open("./" + path, "w").unwrap();
        f.write("Hello World!");
        f.close();

        f = File.open(path).unwrap();
        print(f.read_to_str());
        print(",");
        // make sure nothing changed
        print(f.read_to_str());
        print(",");
        f.close();

        // and again
        f = File.open("./" + path).unwrap();
        print(f.read_to_str());
        print(",");
        print(f.read_to_str());
        print(",");
        f.close();

        f = File.open("./" + path, "w").unwrap();
        // reset
        f.write("");
        print(f.read_to_str());
        f.close();
    }
'

expect '' '
    def main () {
        let path = "./test/spec/std/File/example1.txt";
        let f = File.open(path).unwrap();
        f.write("Hello World!");
        // fail as not opened with write permissions
        print(f.read_to_str());
        f.close();
    }
'