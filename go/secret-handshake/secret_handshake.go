package secret

import (
	"strconv"
)

func Handshake(code uint) []string {
	var h []string
	events := []string{"wink", "double blink", "close your eyes", "jump"}
	codeStr := strconv.FormatInt(int64(code), 2)

	isReverse := false
	if len(codeStr) == 5 {
		isReverse = codeStr[0] == '1'
		codeStr = codeStr[1:]
	}

	for i := 0; i < len(codeStr); i++ {
		if codeStr[len(codeStr)-i-1] == '1' {
			h = append(h, events[i])
		}
	}

	if isReverse {
		for i, j := 0, len(h)-1; i < j; i, j = i+1, j-1 {
			h[i], h[j] = h[j], h[i]
		}
	}

	return h
}
