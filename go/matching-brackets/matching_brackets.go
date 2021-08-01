package brackets

func Bracket(input string) bool {
	matchBraces := map[rune]rune{
		'}': '{',
		')': '(',
		']': '[',
	}
	braces := []rune{}
	for _, c := range input {
		switch c {
		case '{', '(', '[':
			braces = append(braces, c)
		case '}', ')', ']':
			if len(braces) == 0 || matchBraces[c] != braces[len(braces)-1] {
				return false
			}
			braces = braces[:len(braces)-1]
		}
	}
	return len(braces) == 0
}
