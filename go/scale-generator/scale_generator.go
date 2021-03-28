package scale

import (
	"strings"
)

func isSharp(tonic string) bool {
	if strings.HasSuffix(tonic, "#") {
		return true
	}
	for _, v := range []string{"C", "G", "D", "A", "E", "B", "e", "b", "a"} {
		if v == tonic {
			return true
		}
	}
	return false
}

func sort(tonic string, lists []string) []string {
	var sorted []string
	for i, v := range lists {
		if strings.ToLower(v) == strings.ToLower(tonic) {
			sorted = append(sorted, lists[i:len(lists)]...)
			sorted = append(sorted, lists[:i]...)
			return sorted
		}
	}
	return []string{}
}

func Scale(tonic, interval string) []string {
	var scales []string

	var lists []string
	if isSharp(tonic) {
		var sharps = []string{"A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"}
		lists = sort(tonic, sharps)
	} else {
		var flats = []string{"A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"}
		lists = sort(tonic, flats)
	}

	if len(interval) == 0 {
		return lists
	}

	i := 0
	scales = append(scales, lists[0])
	for _, c := range interval[:len(interval)-1] {
		switch c {
		case 'm':
			i++
		case 'M':
			i += 2
		case 'A':
			i += 3
		}
		scales = append(scales, lists[i])
	}

	return scales
}
