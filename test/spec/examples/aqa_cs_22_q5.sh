describe 'AQA Computer Science A-Level Paper 1 2022 Question 5'

expect 'herso geoso ponkfiaryarmidalli nakedmolerat lynx pig' '
    def is_vowel (c: Char) Bool {
        return c == "a".at_raw(0)
            || c == "e".at_raw(0)
            || c == "i".at_raw(0)
            || c == "o".at_raw(0)
            || c == "u".at_raw(0)
    }

    def nth_last_vowel(str: Str, n: Int) Char {
        let mut i = str.len() - 1
        let mut n_th_vowel = 0

        while i >= 0 {
            if is_vowel(str.at_raw(i)) {
                n_th_vowel += 1
                if n_th_vowel == n {
                    return str.at_raw(i)
                }
            }
            i -= 1
        }

        panic("No nth vowel in '" + str + "'")
        return "".at_raw(0)
    }

    def reverse (str: Str) Str {
        let mut res = ""

        let mut i = 0
        let mut nth_vowel = 0

        for char in str {
            if is_vowel(char) {
                nth_vowel += 1
                res += nth_last_vowel(str, nth_vowel).Str()
            } else {
                res += char.Str()
            }
        }

        return res
    }

    print(
        reverse("horse") + " " +
        reverse("goose") + " " +
        reverse("pinkfairyarmadillo") + " " +
        reverse("nakedmolerat") + " " +
        reverse("lynx") + " " +
        reverse("pig")
    )
'