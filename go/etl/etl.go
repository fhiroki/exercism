package etl

import "strings"

// Transform returns ETL letter.
func Transform(old map[int][]string) map[string]int {
	// var scores expectation
	scores := make(map[string]int)
	for k, letters := range old {
		for _, letter := range letters {
			scores[strings.ToLower(letter)] = k
		}
	}
	return scores
}
