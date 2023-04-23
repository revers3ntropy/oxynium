describe 'class List'

expect '732truefalsetrue' '
    def main () {
    	let l = new List<Int> {
    		head: Ptr.make!<Int>(7),
    		size: 8
    	};
    	l.push(2);
    	l.push(3);
    	print(l.at(0).unwrap().Str());
    	print(l.at(2).unwrap().Str());
    	print(l.at(1).unwrap().Str());
    	print(l.at(1).is_some().Str());
    	print(l.at(3).is_some().Str());
    	print(l.at(-1).is_some().Str());
    }
'
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
    	l.push(2, 6)
    	print(l.at(0).unwrap().Str())
    	print(l.len().Str())
    }
'


describe 'List.set_at'

expect 'false,false,false,2,true,1,false,true,3,false' '
    def main () {
    	let l = List.empty!<Int>()
    	// cannot set anything in empty list
    	print(l.set_at(0, 2).is_ok().Str())
    	print(",")
    	print(l.set_at(1, 2).is_ok().Str())
    	print(",")
    	print(l.set_at(-1, 2).is_ok().Str())

    	l.push(2)

    	print(",")
    	print(l.at(0).unwrap().Str())
    	print(",")
    	print(l.set_at(0, 1).is_ok().Str())
    	print(",")
    	print(l.at(0).unwrap().Str())
    	print(",")
    	print(l.set_at(1, 1).is_ok().Str())
    	print(",")
    	print(l.set_at(-1, 3).is_ok().Str())
        print(",")
        print(l.at(0).unwrap().Str())
    	print(",")
    	print(l.set_at(10, 1).is_ok().Str())
    }
'