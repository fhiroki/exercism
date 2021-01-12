package isogram

import "strings"

// IsIsogram determines if a word or phrase is an isogram.
func IsIsogram(s string) bool {
	s = strings.ReplaceAll(s, "-", "")
	s = strings.ReplaceAll(s, " ", "")
	s = strings.ToLower(s)
	n := len(s)

	for i := 0; i < n-1; i++ {
		for j := i + 1; j < n; j++ {
			if s[i] == s[j] {
				return false
			}
		}
	}
	return true
}
