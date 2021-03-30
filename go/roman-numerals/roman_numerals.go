package romannumerals

import "errors"

type Conversion struct {
	value int
	digit string
}

var conversions = []Conversion{
	{value: 1000, digit: "M"},
	{value: 900, digit: "CM"},
	{value: 500, digit: "D"},
	{value: 400, digit: "CD"},
	{value: 100, digit: "C"},
	{value: 90, digit: "XC"},
	{value: 50, digit: "L"},
	{value: 40, digit: "XL"},
	{value: 10, digit: "X"},
	{value: 9, digit: "IX"},
	{value: 5, digit: "V"},
	{value: 4, digit: "IV"},
	{value: 1, digit: "I"},
}

func ToRomanNumeral(arabic int) (string, error) {
	if arabic <= 0 || arabic > 3000 {
		return "", errors.New("arabic must be greater than 0 or less than equal 3000.")
	}

	roman := ""
	for _, conversion := range conversions {
		for arabic >= conversion.value {
			roman += conversion.digit
			arabic -= conversion.value
		}
	}

	return roman, nil
}
