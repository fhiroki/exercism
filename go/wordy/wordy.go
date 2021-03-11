package wordy

import (
	"strconv"
	"strings"
)

func toInt(word string) int {
	if strings.HasSuffix(word, "?") {
		word = word[:len(word)-1]
	}
	v, err := strconv.Atoi(word)
	if err != nil {
		return 0
	}
	return v
}

// Answer parses and evaluate math word problems.
func Answer(quesion string) (int, bool) {
	words := strings.Split(quesion, " ")[2:]
	if len(words) == 0 {
		return 0, false
	}

	var answer int = toInt(words[0])
	for i := 1; i < len(words); i++ {
		switch words[i] {
		case "plus":
			answer += toInt(words[i+1])
			i++
		case "minus":
			answer -= toInt(words[i+1])
			i++
		case "multiplied":
			answer *= toInt(words[i+2])
			i += 2
		case "divided":
			answer /= toInt(words[i+2])
			i += 2
		default:
			return 0, false
		}
	}
	return answer, true
}
