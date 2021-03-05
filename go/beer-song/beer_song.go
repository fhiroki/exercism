package beer

import (
	"errors"
	"fmt"
)

// Verse returns one verse.
func Verse(n int) (string, error) {
	if n > 99 {
		return "", errors.New("There are only 99 bottles")
	}

	verse := "%s of beer on the wall, %s of beer.\n"
	if n == 0 {
		verse = fmt.Sprintf(verse, "No more bottles", "no more bottles")
		verse += "Go to the store and buy some more, 99 bottles of beer on the wall.\n"
	} else if n == 1 {
		verse = fmt.Sprintf(verse, "1 bottle", "1 bottle")
		verse += "Take it down and pass it around, no more bottles of beer on the wall.\n"
	} else {
		bottle := fmt.Sprintf("%d bottles", n)
		verse = fmt.Sprintf(verse, bottle, bottle)

		verse += "Take one down and pass it around, %s of beer on the wall.\n"
		if n == 2 {
			verse = fmt.Sprintf(verse, "1 bottle")
		} else {
			verse = fmt.Sprintf(verse, fmt.Sprintf("%d bottles", n-1))
		}
	}
	return verse, nil
}

// Verses returns several verses.
func Verses(upper, lower int) (string, error) {
	if upper > 99 {
		return "", errors.New("upper must be less than 99")
	} else if lower < 0 {
		return "", errors.New("lower must be greater than 0")
	} else if lower > upper {
		return "", errors.New("lower must be greater than upper")
	}

	var verses string
	for i := upper; i >= lower; i-- {
		verse, err := Verse(i)
		if err == nil {
			verses += verse + "\n"
		}
	}
	return verses, nil
}

// Song returns complete lyrics.
func Song() string {
	var verses string
	for i := 99; i >= 0; i-- {
		verse, err := Verse(i)
		if err == nil {
			verses += verse + "\n"
		}
	}
	return verses
}
