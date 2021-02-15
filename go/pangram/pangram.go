package pangram

import "strings"

// IsPangram determines if a sentence is a pangram.
func IsPangram(sentence string) bool {
	if len(sentence) == 0 {
		return false
	}
	sentence = strings.ToLower(sentence)

	letters := map[rune]bool{}
	for _, v := range sentence {
		letters[v] = true
	}
	for i := 'a'; i <= 'z'; i++ {
		if !letters[i] {
			return false
		}
	}
	return true
}
