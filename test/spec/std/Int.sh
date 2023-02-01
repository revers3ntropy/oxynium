describe 'primitive Int'

expect '0' 'print(new Int.Str())'

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
    print((2 - 1).Str());
    print((2 + 1).Str());
    print((1 - 2).Str());
    print((1 + 2).Str());
    print((2 + 0).Str());
    const a = 9;
    print((0 + a + 2).Str());
    print((a - 0).Str());
    print((0 - 2).Str());
'
expect '10003200' '
    print((1 * 1).Str());
    print((1 * 0).Str());
    print((0 * 1).Str());
    print((0 * 0).Str());
    print((1 * 3).Str());
    print((2 * 1).Str());
    print((2 * 0).Str());
    print((0 * 2).Str());
'
expect '10020' '
    print((1 / 1).Str());
    print((0 / 1).Str());
    print((1 / 3).Str());
    print((2 / 1).Str());
    print((0 / 2).Str());
'
expect_err 'TypeError' '1 / 0'
expect_err 'TypeError' '0 / 0'


describe 'Extreme Int Values'

expect_expr_int '9223372036854775806' '9223372036854775806' # 2^(64-1)-2, max int size
expect_expr_int '9223372036854775807' '9223372036854775807' # 2^(64-1)-1, max int size
expect_err 'NumericOverflow' '9223372036854775808' # 2^63, too big for int
expect_err 'NumericOverflow' '34567856785678995678'
expect_err 'NumericOverflow' '999999999999999999999999999999999999999999999999999999999999'
expect_err 'NumericOverflow' 'const n = 9999999999999999999999999999999999999999999999999999999999999'
expect_expr_int '-9223372036854775808' '9223372036854775807+1' # overflow to -max
expect_expr_int '-9223372036854775807' '9223372036854775807+2' # overflow to -max+1
expect_expr_int '-9223372036854775806' '9223372036854775807+3' # overflow to -max+2


describe 'func Int.Str'

expect '1'           'print(1.Str())'
expect '123'         'print(123.Str())'
expect '1234567890'  'print(1234567890.Str())'
expect '0'           'print(0.Str())'
expect '-1'          'print((-1).Str())'
expect '-123'        'print((-123).Str())'
expect '-1234567890' 'print((-1234567890).Str())'
expect '0' '
    const a = 0;
    print(a.Str());
'
expect '-106709' '
    func f() {
        let a = -106709;
        let a_str = a.Str();
        print(a_str);
    };
    f();
'


describe 'func Int.max'

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


describe 'func Int.min'

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


describe 'func Int.>'

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
expect_err 'TypeError' 'func a(); a > 4'


describe 'func Int.<'

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
expect_err 'TypeError' 'func a(); a < 4'


describe 'func Int.>='

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
expect_err 'TypeError' 'func a(); a >= 4'


describe 'func Int.<='

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
expect_err 'TypeError' 'func a(); a <= 4'


describe 'func Int.=='

expect 'false,false,true,false,false,false,false,false,false' '
    print((1 == 2).Str());
    print(",");
    print((2 == 1).Str());
    print(",");
    print((1 == 1).Str());
    print(",");
    print((1 == 0).Str());
    print(",");
    print((0 == 1).Str());
    print(",");
    print((1 == -1).Str());
    print(",");
    print((-1 == 1).Str());
    print(",");
    print((1 + 4 == 2).Str());
    print(",");
    print((1 + 4 == 2 * 3).Str());
'
expect_err 'TypeError' 'true == 2'
expect_err 'TypeError' '1 == 2 == 4'
expect_err 'TypeError' '2 == ""'
expect_err 'TypeError' '2 == Int'


describe 'func Int.!='

expect 'true,true,false,true,true,true,true,true,true' '
    print((1 != 2).Str());
    print(",");
    print((2 != 1).Str());
    print(",");
    print((1 != 1).Str());
    print(",");
    print((1 != 0).Str());
    print(",");
    print((0 != 1).Str());
    print(",");
    print((1 != -1).Str());
    print(",");
    print((-1 != 1).Str());
    print(",");
    print((1 + 4 != 2).Str());
    print(",");
    print((1 + 4 != 2 * 3).Str());
'
expect_err 'TypeError' 'true != 2'
expect_err 'TypeError' '2 != true'
expect_err 'TypeError' '1 != 2 != 4'
expect_err 'TypeError' '2 != ""'
expect_err 'TypeError' '2 != Int'