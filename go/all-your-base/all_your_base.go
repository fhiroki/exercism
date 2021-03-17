package allyourbase

import (
	"errors"
	"math"
)

func pow(x, y int) int {
	return int(math.Pow(float64(x), float64(y)))
}

func reverse(s []int) {
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		s[i], s[j] = s[j], s[i]
	}
}

// ConvertToBase returns converted base number.
func ConvertToBase(inputBase int, digits []int, outputBase int) ([]int, error) {
	if inputBase < 2 {
		return nil, errors.New("input base must be >= 2")
	}
	if outputBase < 2 {
		return nil, errors.New("output base must be >= 2")
	}

	t := 0
	for i := 0; i < len(digits); i++ {
		v := digits[i]
		if v < 0 || v >= inputBase {
			return nil, errors.New("all digits must satisfy 0 <= d < input base")
		}
		t += v * pow(inputBase, len(digits)-1-i)
	}

	var converted []int
	for t >= outputBase {
		converted = append(converted, t%outputBase)
		t /= outputBase
	}
	converted = append(converted, t)
	reverse(converted)

	return converted, nil
}
