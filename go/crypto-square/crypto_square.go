package cryptosquare

import (
	"math"
	"regexp"
	"strings"
)

// Encode returns encoded text.
func Encode(pt string) string {
	reg := regexp.MustCompile(`[^\w]`)
	str := reg.ReplaceAllString(pt, "")
	str = strings.ToLower(str)

	n := len(str)
	r := int(math.Sqrt(float64(n)))
	c := r
	if r*r != n {
		c++
	}

	rows := make([]string, 0)
	for i := 0; i < n; i += c {
		end := int(math.Min(float64(i+c), float64(n)))
		rows = append(rows, str[i:end])
	}

	if n > c*r {
		r++
	}
	ct := ""
	for i := 0; i < c; i++ {
		for j := 0; j < r; j++ {
			if len(rows[j]) > i {
				ct += string(rows[j][i])
			} else {
				ct += " "
			}
		}
		if i != c-1 {
			ct += " "
		}
	}

	return ct
}
