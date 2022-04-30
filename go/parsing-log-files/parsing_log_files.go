package parsinglogfiles

import (
	"fmt"
	"regexp"
	"strings"
)

func IsValidLine(text string) bool {
	codes := []string{"TRC", "DBG", "INF", "WRN", "ERR", "FTL"}
	for _, code := range codes {
		if strings.HasPrefix(text, fmt.Sprintf("[%s]", code)) {
			return true
		}
	}
	return false
}

func SplitLogLine(text string) []string {
	re := regexp.MustCompile(`<[\*~=-]*>`)
	return re.Split(text, -1)
}

func CountQuotedPasswords(lines []string) int {
	var count int
	re := regexp.MustCompile(`(?i)"[\w\s]*password[\w\s]*"`)
	for _, line := range lines {
		if re.MatchString(line) {
			count++
		}
	}
	return count
}

func RemoveEndOfLineText(text string) string {
	re := regexp.MustCompile(`end-of-line\d*`)
	return re.ReplaceAllString(text, "")
}

func TagWithUserName(lines []string) []string {
	var newlines []string
	re := regexp.MustCompile(`User\s+(\w+)`)
	for _, line := range lines {
		group := re.FindStringSubmatch(line)
		newline := line
		if len(group) != 0 {
			newline = fmt.Sprintf("[USR] %s ", group[1]) + line
		}
		newlines = append(newlines, newline)
	}
	return newlines
}
