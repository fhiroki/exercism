package luhn

import (
	"strings"
)

// Reverse returns reversed string.
func Reverse(s string) string {
	r := []rune(s)
	for i, j := 0, len(r)-1; i < j; i, j = i+1, j-1 {
		r[i], r[j] = r[j], r[i]
	}
	return string(r)
}

// Valid determine whether or not it is valid per the Luhn formula.
func Valid(s string) bool {
	s = strings.ReplaceAll(s, " ", "")
	if len(s) <= 1 {
		return false
	}
	s = Reverse(s)

	sum := 0
	for i, c := range s {
		n := int(c - '0')
		if 0 <= n && n <= 9 {
			if i%2 == 1 {
				sum += n * 2
				if n >= 5 {
					sum -= 9
				}
			} else {
				sum += n
			}
		} else {
			return false
		}
	}

	if sum%10 == 0 {
		return true
	}
	return false
}
