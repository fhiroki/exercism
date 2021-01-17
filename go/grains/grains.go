package grains

import (
	"errors"
	"fmt"
	"os"
)

// Square returns how many grains.
func Square(n int) (uint64, error) {
	if n <= 0 || n > 64 {
		return 0, errors.New("square must be greater than 0 and less than 64")
	}
	var grain uint64 = 1
	for i := 0; i < n-1; i++ {
		grain *= 2
	}
	return grain, nil
}

// Total returns sum grains.
func Total() uint64 {
	var sum uint64 = 0
	for i := 1; i <= 64; i++ {
		grain, err := Square(i)
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		sum += grain
	}
	return sum
}
