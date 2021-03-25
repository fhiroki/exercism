package atbash

import (
	"regexp"
	"strings"
)

func Atbash(s string) string {
	reg := regexp.MustCompile(`[^\w]`)
	s = reg.ReplaceAllString(s, "")

	var encoded string
	for i, r := range strings.ToLower(s) {
		if i != 0 && i%5 == 0 {
			encoded += " "
		}

		n := r - 'a'
		if n >= 0 && n <= 25 {
			encoded += string(25 - n + 'a')
		} else {
			encoded += string(r)
		}
	}
	return encoded
}
