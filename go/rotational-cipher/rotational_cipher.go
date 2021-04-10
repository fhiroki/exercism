package rotationalcipher

import (
	"unicode"
)

func RotationalCipher(plain string, shift int) string {
	cipher := ""
	for _, c := range plain {
		if unicode.IsLetter(c) {
			a := 'a'
			if unicode.IsUpper(c) {
				a = 'A'
			}
			v := int(c-a) + shift
			v %= 26
			cipher += string(rune(v) + a)
		} else {
			cipher += string(c)
		}
	}
	return cipher
}
