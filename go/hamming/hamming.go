package hamming

import "errors"

// Distance calc hamming distance of two strings.
func Distance(a, b string) (int, error) {
	if len(a) != len(b) {
		return 0, errors.New("input must have the same length")
	}
	dist := 0
	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			dist++
		}
	}
	return dist, nil
}
