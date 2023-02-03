describe 'Composition with Classes'

expect '13' '
    class S {
        x: Int,
        y: Int
    }
    class S2 {
        s: S,
        z: Int
    }
    def main () {
        let s2 = new S2 {
            s: new S {
                x: 1, y: 2
            },
            z: 3
        };
        print(s2.s.x.Str());
        print(s2.z.Str());
    }
'

expect '12' '
    class S2 {
        x: Int
    }
    class S {
        x: Int,
        def make_s2(self) S2 {
            return new S2 {
                x: self.x
            }
        }
    }
    def main () {
        print(new S { x: 1 }.make_s2().x.Str());
        let s = new S { x: 2 };
        print(s.make_s2().x.Str());
    }
'
