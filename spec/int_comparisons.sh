describe 'GT'

expect '1 > 2' '0'
expect '2 > 1' '1'
expect '1 > 1' '0'
expect '1 > 0' '1'
expect '0 > 1' '0'
expect '1 > -1' '1'
expect '-1 > 1' '0'
expect '1 + 4 > 2' '1'
expect '1 + 4 > 2 * 3' '0'


describe 'LT'

expect '1 < 2' '1'
expect '2 < 1' '0'
expect '1 < 1' '0'
expect '1 < 0' '0'
expect '0 < 1' '1'
expect '1 < -1' '0'
expect '-1 < 1' '1'
expect '1 + 4 < 2' '0'
expect '1 + 4 < 2 * 3' '1'


describe 'GE'

expect '1 >= 2' '0'
expect '2 >= 1' '1'
expect '1 >= 1' '1'
expect '1 >= 0' '1'
expect '0 >= 1' '0'
expect '1 >= -1' '1'
expect '-1 >= 1' '0'
expect '1 + 4 >= 2' '1'
expect '1 + 4 >= 2 * 3' '0'


describe 'LE'

expect '1 <= 2' '1'
expect '2 <= 1' '0'
expect '1 <= 1' '1'
expect '1 <= 0' '0'
expect '0 <= 1' '1'
expect '1 <= -1' '0'
expect '-1 <= 1' '1'
expect '1 + 4 <= 2' '0'
expect '1 + 4 <= 2 * 3' '1'


describe 'EQ'

expect '1 == 2' '0'
expect '2 == 1' '0'
expect '1 == 1' '1'
expect '1 == 0' '0'
expect '0 == 1' '0'
expect '1 == -1' '0'
expect '-1 == 1' '0'
expect '1 + 4 == 2' '0'
expect '1 + 4 == 2 * 3' '0'


describe 'NE'

expect '1 != 2' '1'
expect '2 != 1' '1'
expect '1 != 1' '0'
expect '1 != 0' '1'
expect '0 != 1' '1'
expect '1 != -1' '1'
expect '-1 != 1' '1'
expect '1 + 4 != 2' '1'
expect '1 + 4 != 2 * 3' '1'
