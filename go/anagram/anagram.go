package anagram

import (
	"sort"
	"strings"
)

// Detect returns anagrams.
func Detect(subject string, candidates []string) []string {
	var anagrams []string
	for _, candidate := range candidates {
		if strings.ToLower(subject) != strings.ToLower(candidate) &&
			sortString(subject) == sortString(candidate) {
			anagrams = append(anagrams, candidate)
		}
	}
	return anagrams
}

func sortString(w string) string {
	w = strings.ToLower(w)
	s := strings.Split(w, "")
	sort.Strings(s)
	return strings.Join(s, "")
}
