package prime

// Factors computes the prime factors.
func Factors(n int64) []int64 {
	// var factors []int64
	factors := make([]int64, 0)
	var i int64

	for n > 1 {
		for i = 2; i <= n; i++ {
			if n%i == 0 {
				n /= i
				factors = append(factors, i)
				break
			}
		}
	}

	return factors
}
