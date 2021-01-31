package lsproduct

import "errors"

// LargestSeriesProduct calculate the largest product for a contiguous substring of digits.
func LargestSeriesProduct(digits string, span int) (int64, error) {
	if len(digits) < span {
		return -1, errors.New("span must be smaller than string length")
	}
	if span < 0 {
		return -1, errors.New("span must be greater than zero")
	}

	var max int64 = 0
	for i := 0; i <= len(digits)-span; i++ {
		var prod int64 = 1
		for j := 0; j < span; j++ {
			digit := int64(digits[i+j] - '0')
			if digit < 0 || digit > 9 {
				return -1, errors.New("digits input must only contain digits")
			}
			prod *= digit
		}
		if max < prod {
			max = prod
		}
	}
	return max, nil
}
