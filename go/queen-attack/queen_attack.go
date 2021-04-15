package queenattack

import (
	"errors"
	"math"
)

func CanQueenAttack(w, b string) (bool, error) {
	if len(w) != 2 || len(b) != 2 {
		return false, errors.New("invalid input")
	}
	if w == b {
		return false, errors.New("same square")
	}

	w1, w2 := float64(w[0]-'a'+1), float64(w[1]-'0')
	b1, b2 := float64(b[0]-'a'+1), float64(b[1]-'0')

	if w1 <= 0 || w1 > 8 || w2 <= 0 || w2 > 8 ||
		b1 <= 0 || b1 > 8 || b2 <= 0 || b2 > 8 {
		return false, errors.New("out of range")
	}

	if w1 == b1 || w2 == b2 || math.Abs(w2-w1) == math.Abs(b2-b1) {
		return true, nil
	}

	return false, nil
}
