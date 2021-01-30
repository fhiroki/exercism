package wordcount

import (
	"regexp"
	"strings"
)

// Frequency is word frequency.
type Frequency map[string]int

// WordCount counts the occurrences of each word in phrase.
func WordCount(words string) Frequency {
	reg := regexp.MustCompile("[^a-zA-Z0-9']+")
	words = reg.ReplaceAllString(words, " ")
	words = strings.ToLower(words)

	freq := make(Frequency)
	for _, word := range strings.FieldsFunc(words, Split) {
		if strings.HasPrefix(word, "'") {
			freq[word[1:len(word)-1]]++
		} else {
			freq[word]++
		}
	}
	return freq
}

// Split determines split rule.
func Split(r rune) bool {
	return r == ' ' || r == ',' || r == '\t' || r == '\n'
}
