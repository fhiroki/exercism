package darts

import "math"

// Score returns the earned points in Darts game.
func Score(x, y float64) int {
	dist := math.Sqrt(x*x + y*y)
	if dist <= 1 {
		return 10
	} else if dist <= 5 {
		return 5
	} else if dist <= 10 {
		return 1
	}
	return 0
}
