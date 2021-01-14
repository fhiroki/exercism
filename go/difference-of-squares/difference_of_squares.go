package diffsquares

// SquareOfSum returns the square of the sum.
func SquareOfSum(n int) int {
	sum := n * (n + 1) / 2
	return sum * sum
}

// SumOfSquares returns the sum of the squares.
func SumOfSquares(n int) int {
	return n * (n + 1) * (2*n + 1) / 6
}

// Difference returns diff between the square of the sum and the sum of the squares.
func Difference(n int) int {
	return SquareOfSum(n) - SumOfSquares(n)
}
