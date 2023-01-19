describe 'primitive Int'

expect '0' 'print(new Int.str())'

expect_err 'TypeError' '0 && 1'
expect_err 'TypeError' '!8'
expect_err 'TypeError' '!0'
expect_err 'TypeError' '0 || 1'

describe 'Arithmetic'

expect_expr_int '1'     '1'
expect_expr_int '2'     '1+1'
expect_expr_int '4'     '1+3'
expect_expr_int '5'     '1+1+1+1+1'
expect_expr_int '9'     '   1 + 1 + 1 + 1 + 3+ 2  '
expect_expr_int '58601' '58600+1'

expect_expr_int '1'      '2-1'
expect_expr_int '0'      '1-1'
expect_expr_int '-2'     '1-3'
expect_expr_int '-3'     '1-1-1-1-1'
expect_expr_int '58599' '58600-1'

expect_expr_int '1' '1*1'
expect_expr_int '3' '1*3'
expect_expr_int '1' '1*1*1*1*1'
expect_expr_int '58600' '58600*1'
expect_expr_int '24' '3*8'

expect_expr_int '1' '1/1'
expect_expr_int '0' '1/3'
expect_expr_int '1' '1/1/1/1/1'
expect_expr_int '58600' '58600/1'
expect_expr_int '3' '24/8'

expect_expr_int '0' '1%1'
expect_expr_int '1' '1%3'
expect_expr_int '1' '3%2'
expect_expr_int '0' '1%1%1%1%1'
expect_expr_int '0' '58600%1'
expect_expr_int '0' '24%8'
expect_expr_int '2' '3%2*2'
expect_expr_int '30033' '30033%876542'
expect_expr_int '3' '3%(2*2)'
expect_expr_int '7' '7/8+7%8'

expect_expr_int '0' '(-0)'
expect_expr_int '-1' '(-1)'
expect_expr_int '-3' '(-3)'
expect_expr_int '-5' '(-1-1-1-1-1)'

expect_expr_int '27' '1+2*3+4*5'
expect_expr_int '-3' '1+2*3-4*5/2'
expect_expr_int '-96' '0+-1*2+3*-8/-4-100'

expect_expr_int '3'  '(1+2)'
expect_expr_int '9'  '(1+2)*3'
expect_expr_int '11' '1+(2*3)+4'
expect_expr_int '27' '1+((2*3)+4*5)'

# as these get optimized, make sure they are working
expect '13-132119-2' '
    print((2 - 1).str());
    print((2 + 1).str());
    print((1 - 2).str());
    print((1 + 2).str());
    print((2 + 0).str());
    const a = 9;
    print((0 + a + 2).str());
    print((a - 0).str());
    print((0 - 2).str());
'
expect '10003200' '
    print((1 * 1).str());
    print((1 * 0).str());
    print((0 * 1).str());
    print((0 * 0).str());
    print((1 * 3).str());
    print((2 * 1).str());
    print((2 * 0).str());
    print((0 * 2).str());
'
expect '10020' '
    print((1 / 1).str());
    print((0 / 1).str());
    print((1 / 3).str());
    print((2 / 1).str());
    print((0 / 2).str());
'
expect_err 'TypeError' '1 / 0'
expect_err 'TypeError' '0 / 0'


describe 'Extreme Int Values'

expect_expr_int '9223372036854775806' '9223372036854775806' # 2^(64-1)-2, max int size
expect_expr_int '9223372036854775807' '9223372036854775807' # 2^(64-1)-1, max int size
expect_err 'NumericOverflow' '9223372036854775808' # 2^63, too big for int
expect_err 'NumericOverflow' '34567856785678995678'
expect_err 'NumericOverflow' '999999999999999999999999999999999999999999999999999999999'
expect_err 'NumericOverflow' 'const n = 999999999999999999999999999999999999999999999999999999999'
expect_expr_int '-9223372036854775808' '9223372036854775807+1' # overflow to -0
expect_expr_int '-9223372036854775807' '9223372036854775807+2' # overflow to negative max
expect_expr_int '-9223372036854775806' '9223372036854775807+3' # overflow to negative max


describe 'fn Int.str'

expect '1'           'print(1.str())'
expect '123'         'print(123.str())'
expect '1234567890'  'print(1234567890.str())'
expect '0'           'print(0.str())'
expect '-1'          'print((-1).str())'
expect '-123'        'print((-123).str())'
expect '-1234567890' 'print((-1234567890).str())'
expect '0' '
    const a = 0;
    print(a.str());
'
expect '-106709' '
    fn f() {
        let a = -106709;
        let a_str = a.str();
        print(a_str);
    };
    f();
'


describe 'fn Int.max'

expect_expr_int '2'   '1.max(2)'
expect_expr_int '2'   '2.max(1)'
expect_expr_int '2'   '2.max(2)'
expect_expr_int '-1'  '-1.max(0)'
expect_expr_int '0'   '(-1).max(0)'
expect_expr_int '1'   '(-1).max(1)'
expect_expr_int '-10' '(-100).max(-10)'
expect_expr_int '-10' '(-10).max(-100)'
expect_expr_int '0'   '0.max(0)'
expect_expr_int '1'   '0.max(1)'
expect_expr_int '1'   '1.max(0)'
expect_expr_int '1'   '1.max(1)'
expect_expr_int '9223372036854775807'   '0.max()'
expect_expr_int '9223372036854775807'   '1.max()'
expect_expr_int '9223372036854775807'   '9223372036854775807.max()'
expect_expr_int '9223372036854775807'  '(-9223372036854775807).max()'

describe 'fn Int.min'

expect_expr_int '1'    '1.min(2)'
expect_expr_int '1'    '2.min(1)'
expect_expr_int '2'    '2.min(2)'
expect_expr_int '-1'   '(-1).min(0)'
expect_expr_int '-1'   '(-1).min(1)'
expect_expr_int '-100' '(-100).min(-10)'
expect_expr_int '0'    '0.min(1)'
expect_expr_int '0'    '1.min(0)'
expect_expr_int '0'    '0.min(0)'
expect_expr_int '1'    '1.min(1)'
expect_expr_int '-9223372036854775808'   '0.min()'
expect_expr_int '-9223372036854775808'   '(-1).min()'
expect_expr_int '-9223372036854775808'   '(-9223372036854775807).min()'
expect_expr_int '-9223372036854775808'  '9223372036854775807.min()'

describe 'fn Int.>'

expect_expr_bool 'false' '1 > 2'
expect_expr_bool 'true'  '2 > 1'
expect_expr_bool 'false' '1 > 1'
expect_expr_bool 'true'  '1 > 0'
expect_expr_bool 'false' '0 > 1'
expect_expr_bool 'true'  '1 > -1'
expect_expr_bool 'false' '-1 > 1'
expect_expr_bool 'true'  '1 + 4 > 2'
expect_expr_bool 'false' '1 + 4 > 2 * 3'
expect_err 'TypeError' 'true > 2'
expect_err 'TypeError' '1 > 2 > 4'
expect_err 'TypeError' '2 > ""'
expect_err 'TypeError' '"" > ""'
expect_err 'TypeError' '"" > true'
expect_err 'TypeError' '2 > false'
expect_err 'TypeError' '"" > 2'
expect_err 'TypeError' 'fn a(); a > 4'


describe 'fn Int.<'

expect_expr_bool 'true'  '1 < 2'
expect_expr_bool 'false' '2 < 1'
expect_expr_bool 'false' '1 < 1'
expect_expr_bool 'false' '1 < 0'
expect_expr_bool 'true'  '0 < 1'
expect_expr_bool 'false' '1 < -1'
expect_expr_bool 'true'  '-1 < 1'
expect_expr_bool 'false' '1 + 4 < 2'
expect_expr_bool 'true'  '1 + 4 < 2 * 3'
expect_err 'TypeError' 'true < 2'
expect_err 'TypeError' '1 < 2 < 4'
expect_err 'TypeError' '2 < ""'
expect_err 'TypeError' '"" < ""'
expect_err 'TypeError' '"" < true'
expect_err 'TypeError' '2 < false'
expect_err 'TypeError' '"" < 2'
expect_err 'TypeError' 'fn a(); a < 4'


describe 'fn Int.>='

expect_expr_bool 'false' '1 >= 2'
expect_expr_bool 'true'  '2 >= 1'
expect_expr_bool 'true'  '1 >= 1'
expect_expr_bool 'true'  '1 >= 0'
expect_expr_bool 'false' '0 >= 1'
expect_expr_bool 'true'  '1 >= -1'
expect_expr_bool 'false' '-1 >= 1'
expect_expr_bool 'true'  '1 + 4 >= 2'
expect_expr_bool 'false' '1 + 4 >= 2 * 3'
expect_err 'TypeError' 'true >= 2'
expect_err 'TypeError' '1 >= 2 >= 4'
expect_err 'TypeError' '2 >= ""'
expect_err 'TypeError' '"" >= ""'
expect_err 'TypeError' '"" >= true'
expect_err 'TypeError' '2 >= false'
expect_err 'TypeError' '"" >= 2'
expect_err 'TypeError' 'fn a(); a >= 4'


describe 'fn Int.<='

expect_expr_bool 'true'  '1 <= 2'
expect_expr_bool 'false' '2 <= 1'
expect_expr_bool 'true'  '1 <= 1'
expect_expr_bool 'false' '1 <= 0'
expect_expr_bool 'true'  '0 <= 1'
expect_expr_bool 'false' '1 <= -1'
expect_expr_bool 'true'  '-1 <= 1'
expect_expr_bool 'false' '1 + 4 <= 2'
expect_expr_bool 'true'  '1 + 4 <= 2 * 3'
expect_err 'TypeError' 'true <= 2'
expect_err 'TypeError' '1 <= 2 <= 4'
expect_err 'TypeError' '2 <= ""'
expect_err 'TypeError' '"" <= ""'
expect_err 'TypeError' '"" <= true'
expect_err 'TypeError' '2 <= false'
expect_err 'TypeError' '"" <= 2'
expect_err 'TypeError' 'fn a(); a <= 4'


describe 'fn Int.=='

expect_expr_bool 'false' '1 == 2'
expect_expr_bool 'false' '2 == 1'
expect_expr_bool 'true' '1 == 1'
expect_expr_bool 'false' '1 == 0'
expect_expr_bool 'false' '0 == 1'
expect_expr_bool 'false' '1 == -1'
expect_expr_bool 'false' '-1 == 1'
expect_expr_bool 'false' '1 + 4 == 2'
expect_expr_bool 'false' '1 + 4 == 2 * 3'
expect_err 'TypeError' 'true == 2'
expect_err 'TypeError' '1 == 2 == 4'
expect_err 'TypeError' '2 == ""'
expect_err 'TypeError' '2 == Int'


describe 'fn Int.!='

expect_expr_bool 'true' '1 != 2'
expect_expr_bool 'true' '2 != 1'
expect_expr_bool 'false' '1 != 1'
expect_expr_bool 'true' '1 != 0'
expect_expr_bool 'true' '0 != 1'
expect_expr_bool 'true' '1 != -1'
expect_expr_bool 'true' '-1 != 1'
expect_expr_bool 'true' '1 + 4 != 2'
expect_expr_bool 'true' '1 + 4 != 2 * 3'
expect_err 'TypeError' 'true != 2'
expect_err 'TypeError' '2 != true'
expect_err 'TypeError' '1 != 2 != 4'
expect_err 'TypeError' '2 != ""'
expect_err 'TypeError' '2 != Int'