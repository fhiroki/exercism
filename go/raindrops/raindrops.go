package raindrops

import "strconv"

// Convert returns raindrops
func Convert(n int) string {
	ans := ""
	if n%3 == 0 {
		ans += "Pling"
	}
	if n%5 == 0 {
		ans += "Plang"
	}
	if n%7 == 0 {
		ans += "Plong"
	}
	if ans == "" {
		ans = strconv.Itoa(n)
	}
	return ans
}
