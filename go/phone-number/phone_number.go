package phonenumber

import (
	"errors"
	"fmt"
	"regexp"
	"strings"
)

func Number(input string) (string, error) {
	input = strings.ReplaceAll(input, " ", "")
	rep := regexp.MustCompile(`[^0-9]`)
	input = rep.ReplaceAllString(input, "")

	if strings.HasPrefix(input, "1") {
		input = input[1:]
	}
	if len(input) != 10 {
		return "", errors.New("invalid number length")
	}
	if input[0] == '0' || input[0] == '1' {
		return "", errors.New("area code cannot start with zero or one")
	}
	if input[3] == '0' || input[3] == '1' {
		return "", errors.New("exchange code cannot start with zero or one")
	}

	return input, nil
}

func AreaCode(input string) (string, error) {
	number, err := Number(input)
	if err != nil {
		return "", err
	}
	return number[:3], nil
}

func Format(input string) (string, error) {
	number, err := Number(input)
	if err != nil {
		return "", err
	}
	return fmt.Sprintf("(%s) %s-%s", number[:3], number[3:6], number[6:]), nil
}
