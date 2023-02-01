describe 'class List'

expect '732truefalsetrue' '
    func main () {
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
    func main () {
    	let l = List.empty!<Int>();
    	print(l.len().Str());
    	l.push(2);
    	l.push(3);
    	print(l.len().Str());
    }
'