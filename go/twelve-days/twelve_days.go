package twelve

import (
	"fmt"
	"strings"
)

type phrase struct {
	number string
	words  string
}

// Song returns Christmas song.
func Song() string {
	var verses []string
	for i := 1; i <= 12; i++ {
		verses = append(verses, Verse(i))
	}
	return strings.Join(verses, "\n")
}

// Verse returns nth day verse.
func Verse(n int) string {
	verse := [...]phrase{
		{"first", "a Partridge in a Pear Tree."},
		{"second", "two Turtle Doves"},
		{"third", "three French Hens"},
		{"fourth", "four Calling Birds"},
		{"fifth", "five Gold Rings"},
		{"sixth", "six Geese-a-Laying"},
		{"seventh", "seven Swans-a-Swimming"},
		{"eighth", "eight Maids-a-Milking"},
		{"ninth", "nine Ladies Dancing"},
		{"tenth", "ten Lords-a-Leaping"},
		{"eleventh", "eleven Pipers Piping"},
		{"twelfth", "twelve Drummers Drumming"},
	}

	lyric := fmt.Sprintf("On the %s day of Christmas my true love gave to me: ", verse[n-1].number)

	if n == 1 {
		return lyric + verse[0].words
	}

	for i := n - 1; i >= 0; i-- {
		if i == 0 {
			lyric += "and " + verse[i].words
		} else {
			lyric += verse[i].words + ", "
		}
	}

	return lyric
}
