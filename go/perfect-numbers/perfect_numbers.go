package perfect

import "errors"

type Classification int

const (
	ClassificationDeficient = iota
	ClassificationPerfect
	ClassificationAbundant
)

var ErrOnlyPositive = errors.New("input must be positive")

func Classify(n int64) (Classification, error) {
	if n <= 0 {
		return -1, ErrOnlyPositive
	}

	var sum int64 = 1
	var i int64
	for i = 2; i*i <= n; i++ {
		if n%i == 0 {
			sum += n / i
			if n/i != i {
				sum += i
			}
		}
	}

	if n == 1 {
		sum = 0
	}

	switch {
	case sum == n:
		return ClassificationPerfect, nil
	case sum < n:
		return ClassificationDeficient, nil
	case sum > n:
		return ClassificationAbundant, nil
	}
	return -1, nil
}
