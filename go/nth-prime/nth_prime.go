package prime

func isPrime(n int) bool {
	for i := 2; i*i <= n; i++ {
		if n%i == 0 {
			return false
		}
	}
	return true
}

// Nth returns nth prime number.
func Nth(n int) (int, bool) {
	if n <= 0 {
		return 0, false
	}
	var primes []int
	for i := 2; i < 1e6; i++ {
		if isPrime(i) {
			primes = append(primes, i)
		}
	}
	return primes[n-1], true
}
