// This is a "stub" file.  It's a little start on your solution.
// It's not a complete solution though; you have to write some code.

// Package leap should have a package comment that summarizes what it's about.
// https://golang.org/doc/effective_go.html#commentary
package leap

// IsLeapYear returns leap year.
func IsLeapYear(n int) bool {
	if n%4 == 0 {
		if n%400 != 0 && n%100 == 0 {
			return false
		}
		return true
	}
	return false
}
