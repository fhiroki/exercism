package armstrong

import (
	"math"
)

func pow(x, y int) int {
	return int(math.Pow(float64(x), float64(y)))
}

func count(n int) int {
	cnt := 0
	for n > 0 {
		cnt++
		n /= 10
	}
	return cnt
}

func calc(n int) int {
	sum := 0
	size := count(n)
	for n > 0 {
		sum += pow(n%10, size)
		n /= 10
	}
	return sum
}

// IsNumber determines whether a number is an Armstrong number.
func IsNumber(n int) bool {
	return calc(n) == n
}
