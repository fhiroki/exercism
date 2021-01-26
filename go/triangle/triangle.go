// This is a "stub" file.  It's a little start on your solution.
// It's not a complete solution though; you have to write some code.

// Package triangle should have a package comment that summarizes what it's about.
// https://golang.org/doc/effective_go.html#commentary
package triangle

import (
	"math"
	"sort"
)

// Notice KindFromSides() returns this type. Pick a suitable data type.
type Kind string

const (
	// Pick values for the following identifiers used by the test program.
	NaT = Kind("NaT") // not a triangle
	Equ = Kind("Equ") // equilateral
	Iso = Kind("Iso") // isosceles
	Sca = Kind("Sca") // scalene
)

// KindFromSides should have a comment documenting it.
func KindFromSides(a, b, c float64) Kind {
	// Write some code here to pass the test suite.
	// Then remove all the stock comments.
	// They're here to help you get started but they only clutter a finished solution.
	// If you leave them in, reviewers may protest!
	var k Kind
	s := []float64{a, b, c}
	sort.Float64s(s)

	if s[0] <= 0 || math.IsNaN(s[0]) || math.IsInf(s[0], -1) || math.IsInf(s[2], 1) {
		return NaT
	}

	if s[0] == s[1] && s[1] == s[2] {
		k = Equ
	} else if (s[0] == s[1] && s[1] != s[2]) || (s[1] == s[2] && s[0] != s[1]) {
		k = Iso
		if s[0]+s[1] < s[2] {
			k = NaT
		}
	} else if s[0]+s[1] >= s[2] {
		k = Sca
	} else {
		k = NaT
	}

	return k
}
