package scrabble

import "strings"

// Score computes Scrabble score.
func Score(word string) int {
	scoreLetter := map[string]int{
		"AEIOULNRST": 1,
		"DG":         2,
		"BCMP":       3,
		"FHVWY":      4,
		"K":          5,
		"JX":         8,
		"QZ":         10,
	}

	score := 0
	for _, w := range word {
		s := strings.ToUpper(string(w))
		for k, v := range scoreLetter {
			if strings.Contains(k, s) {
				score += v
				break
			}
		}
	}

	return score
}
