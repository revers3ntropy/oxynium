describe 'class List'

expect '732truefalsetrue' '
    def main () {
    	let l = new List<Int> {
    		head: Ptr.make!<Int>(7),
    		length: 1,
    		capacity: 8
    	}
    	l.push(2)
    	l.push(3)
    	print(l.at(0).unwrap().Str())
    	print(l.at(2).unwrap().Str())
    	print(l.at(1).unwrap().Str())
    	print(l.at(1).is_some.Str())
    	print(l.at(3).is_some.Str())
    	print(l.at(-1).is_some.Str())
    }
'

describe 'def List.empty, def List.push'

expect '02' '
    def main () {
    	let l = List.empty!<Int>();
    	print(l.len().Str());
    	l.push(2);
    	l.push(3);
    	print(l.len().Str());
    }
'
expect '26' '
    def main () {
    	let l = List.empty!<Int>()
    	l.push(2)
    	l.push(2)
    	l.push(2)
    	l.push(2)
    	l.push(2)
    	l.push(2)
    	print(l.at(0).unwrap().Str())
    	print(l.len().Str())
    }
'


describe 'def List.with_capacity'

expect '0 0 0 1 1 64 2 2 64 0 0 8 1 1 8 2 16 0 2 1 64' '
    def main () {
        let mut l = List.with_capacity!<Int>(0)
        print(l.length.Str(), " ")
        print(l.len().Str(), " ")
        print(l.capacity.Str(), " ")

        l.push(2)
        print(l.length.Str(), " ")
        print(l.len().Str(), " ")
        print(l.capacity.Str(), " ")

        l.push(3)
        print(l.length.Str(), " ")
        print(l.len().Str(), " ")
        print(l.capacity.Str(), " ")

        l = List.with_capacity!<Int>(8)
        print(l.length.Str(), " ")
        print(l.len().Str(), " ")
        print(l.capacity.Str(), " ")

        l.push(67)
        print(l.length.Str(), " ")
        print(l.len().Str(), " ")
        print(l.capacity.Str(), " ")

        l.push(67)
        print(l.length.Str(), " ")
        print(l.capacity.Str(), " ")

        l = List.with_capacity!<Int>(2)
        print(l.length.Str(), " ")
        print(l.capacity.Str(), " ")
        // when we push to a buffer that has size but the size
        // is not enough, the buffer will be reallocated to the required size
        l.push(1)
        print(l.length.Str(), " ")
        print(l.capacity.Str(), "")
    }
'


describe 'def List.clone'

expect '3 11 12 2 11 14 128' '
    def main () {
        let mut l = List.empty!<Int>()
        l.push(11)
        let mut l2 = l.clone()
        l.push(12)
        l.push(13)
        l2.push(14)
        print(l.length.Str(), " ")
        print(l.at_raw(0).Str(), " ")
        print(l.at_raw(1).Str(), " ")
        // check arrays are decoupled
        print(l2.length.Str(), " ")
        print(l2.at_raw(0).Str(), " ")
        print(l2.at_raw(1).Str(), " ")

        // check cloned arrays get the same capacity
        l = List.with_capacity!<Int>(128)
        print(l.clone().capacity.Str(), "")
    }
'


describe 'def List.concat'

expect '2 3 5 2 3 6 3 3 6 2 3 4 5 6 7' '
    def main () {
        let l1 = List.empty!<Int>()
        l1.push(2)
        l1.push(3)

        let l2 = List.empty!<Int>()
        l2.push(4)
        l2.push(5)
        l2.push(6)

        // create new list with contents of l1 and l2
        let l = l1.concat(l2)

        print(l1.length.Str(), " ")
        print(l2.length.Str(), " ")
        print(l.length.Str(), " ")

        l.push(7)

        print(l1.length.Str(), " ")
        print(l2.length.Str(), " ")
        print(l.length.Str(), " ")

        l1.push(0)

        print(l1.length.Str(), " ")
        print(l2.length.Str(), " ")
        print(l.length.Str(), " ")

        print(l.at_raw(0).Str(), " ")
        print(l.at_raw(1).Str(), " ")
        print(l.at_raw(2).Str(), " ")
        print(l.at_raw(3).Str(), " ")
        print(l.at_raw(4).Str(), " ")
        print(l.at_raw(5).Str(), "")
    }
'


describe 'def List.set_at'

expect 'false false false 2 true 1 false true false 3 false' '
    def main () {
    	let l = List.empty!<Int>()
    	// cannot set anything in empty list
    	print(l.set_at(0, 2).ok.Str(), " ")
    	print(l.set_at(1, 2).ok.Str(), " ")
    	print(l.set_at(-1, 2).ok.Str(), " ")

    	l.push(2)

    	print(l.at(0).unwrap().Str(), " ")
    	print(l.set_at(0, 1).ok.Str(), " ")
    	print(l.at(0).unwrap().Str(), " ")
    	print(l.set_at(1, 1).ok.Str(), " ")
    	print(l.set_at(-1, 3).ok.Str(), " ")
        print(l.set_at(-2, 3).ok.Str(), " ")
        print(l.at(0).unwrap().Str(), " ")
    	print(l.set_at(10, 1).ok.Str(), "")
    }
'


describe 'def List.sort'

expect '1 2 3 4 5 6 6 5 4 3 2 1 ' '
    def int_compare_rev (a: Int, b: Int) Int ->
        Int.compare(b, a)

    def main () {
        let l = List.empty!<Int>()
        l.push(6)
        l.push(2)
        l.push(3)
        l.push(4)
        l.push(5)
        l.push(1)

        for i in l.sort(Int.compare) {
            print(i.Str(), " ")
        }

        for j in l.sort(int_compare_rev) {
            print(j.Str(), " ")
        }
    }
'


describe 'def List.filter'

expect '0 6 12 18 24 30 36 42 48 |||0' '
    def is_multiple_of_6 (a: Int) Bool ->
        a % 6 == 0

    def main () {
        for i in (range(50)).List().filter(is_multiple_of_6) {
            print(i.Str(), " ")
        }
        print("|")
        for j in (range(0)).List().filter(is_multiple_of_6) {
            print(j.Str(), " ")
        }
        print("|")
        for k in List.empty!<Int>().filter(is_multiple_of_6) {
            print(k.Str(), " ")
        }
        print("|")
        for l in (range(6)).List().filter(is_multiple_of_6) {
            print(l.Str())
        }
    }
'
expect '50 0 0 0' '
    def t (_: Int) Bool -> true
    def f (_: Int) Bool -> false

    def main () {
        print(range(50).List().filter(t).len().Str(), " ")
        print(range(0).List().filter(t).len().Str(), " ")
        print(range(50).List().filter(f).len().Str(), " ")
        print(range(0).List().filter(f).len().Str())
    }
'


describe 'List.index_of'

expect 'false false false true true 0 1' '
    def main () {
        let l = List.empty!<Int>()
        print(l.index_of(1).is_some.Str(), " ")
        print(l.index_of(0).is_some.Str(), " ")

        l.push(7)
        l.push(8)

        print(l.index_of(1).is_some.Str(), " ")
        print(l.index_of(7).is_some.Str(), " ")
        print(l.index_of(8).is_some.Str(), " ")
        print(l.index_of(7).unwrap().Str(), " ")
        print(l.index_of(8).unwrap().Str())
    }
'
expect 'false true 1 true 1' '
    def main () {
        let s = "hello"
        let l = List.empty!<Str>()
        l.push("world")
        l.push(s)

        // by default compares pointers
        print(l.index_of("hello").is_some.Str(), " ")
        print(l.index_of(s).is_some.Str(), " ")
        print(l.index_of(s).unwrap().Str(), " ")
        print(l.index_of("hello", fn (a: Str, b: Str) -> a == b).is_some.Str(), " ")
        print(l.index_of("hello", fn (a: Str, b: Str) -> a == b).unwrap().Str())
    }
'


describe 'List.remove_at'

expect 'false 0 1 2 3 4 true 2 4 5 6 7 11' '
    def main () {
        let mut l = List.empty!<Int>()
        print(l.remove_at(1).is_some.Str(), " ")

        l = range(15).List()

        print(l.at(0).unwrap().Str(), " ")
        print(l.at(1).unwrap().Str(), " ")
        print(l.at(2).unwrap().Str(), " ")
        print(l.at(3).unwrap().Str(), " ")
        print(l.at(4).unwrap().Str(), " ")

        print(l.remove_at(0).is_some.Str(), " ")
        l.remove_at(0)
        l.remove_at(1)
        l.remove_at(l.len() - 1)

        print(l.at(0).unwrap().Str(), " ")
        print(l.at(1).unwrap().Str(), " ")
        print(l.at(2).unwrap().Str(), " ")
        print(l.at(3).unwrap().Str(), " ")
        print(l.at(4).unwrap().Str(), " ")

        print(l.len().Str())
    }
'