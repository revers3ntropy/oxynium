describe 'List'

expect '732truefalsetrue' '
    fn main () {
    	let l = new List<Int> {
    		head: new Ptr<Int>.make_from(7),
    		size: 8
    	};
    	l.push(2);
    	l.push(3);
    	print(l.at(0).unwrap().str());
    	print(l.at(2).unwrap().str());
    	print(l.at(1).unwrap().str());
    	print(l.at(1).is_some().str());
    	print(l.at(3).is_some().str());
    	print(l.at(-1).is_some().str());
    }
'
