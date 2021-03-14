package isbn

import "strings"

// IsValidISBN validates book identification numbers.
func IsValidISBN(isbn string) bool {
	isbn = strings.ReplaceAll(isbn, "-", "")
	if len(isbn) != 10 {
		return false
	}

	sum := 0
	for i, c := range isbn {
		val := int(c - '0')
		if val >= 0 && val <= 9 {
			sum += val * (i + 1)
		} else if i == 9 && c == 'X' {
			sum += 10 * (i + 1)
		} else {
			return false
		}
	}
	return sum%11 == 0
}
