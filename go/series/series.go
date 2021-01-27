package series

// All returns contiguous substrings of length n.
func All(n int, s string) []string {
	var series []string
	for i := 0; i < len(s)-n+1; i++ {
		series = append(series, s[i:i+n])
	}
	return series
}

// UnsafeFirst returns first substring.
func UnsafeFirst(n int, s string) string {
	return All(n, s)[0]
}

// First is modified version of UnsafeFirst.
func First(n int, s string) (string, bool) {
	if n > len(s) {
		return "", false
	}
	return All(n, s)[0], true
}
