package thefarm

import (
	"errors"
	"fmt"
)

// See types.go for the types defined for this exercise.

// Define the SillyNephewError type here.
var SillyNephewError = errors.New("negative fodder")

// DivideFood computes the fodder amount per cow for the given cows.
func DivideFood(weightFodder WeightFodder, cows int) (float64, error) {
	fodder, err := weightFodder.FodderAmount()

	if err == ErrScaleMalfunction {
		if fodder > 0 {
			return fodder / float64(cows) * 2.0, nil
		} else {
			return 0.0, SillyNephewError
		}
	} else if err != nil {
		return 0.0, err
	}

	if fodder < 0 {
		return 0.0, SillyNephewError
	}

	if cows == 0 {
		return 0.0, errors.New("division by zero")
	} else if cows < 0 {
		return 0.0, errors.New(fmt.Sprintf("silly nephew, there cannot be %d cows", cows))
	}

	return fodder / float64(cows), nil
}
