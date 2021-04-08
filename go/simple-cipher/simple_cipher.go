package cipher

import (
	"math"
	"regexp"
	"strings"
)

func format(str string) string {
	reg := regexp.MustCompile(`[^a-z]`)
	str = strings.ToLower(str)
	str = reg.ReplaceAllString(str, "")
	return str
}

func code(str string, shift int) string {
	str = format(str)
	cipher := ""
	for _, s := range str {
		v := s + rune(shift)
		if v-'a' >= 26 {
			v -= 26
		} else if v-'a' < 0 {
			v += 26
		}
		cipher += string(v)
	}
	return cipher
}

type CaesarCipher struct{}

func (c CaesarCipher) Encode(str string) string {
	return code(str, 3)
}

func (c CaesarCipher) Decode(str string) string {
	return code(str, -3)
}

func NewCaesar() Cipher {
	var c CaesarCipher
	return c
}

type ShiftCipher struct {
	distance int
}

func (c ShiftCipher) Encode(str string) string {
	return code(str, c.distance)
}

func (c ShiftCipher) Decode(str string) string {
	return code(str, -c.distance)
}

func NewShift(distance int) Cipher {
	if math.Abs(float64(distance)) >= 26 || distance == 0 {
		return nil
	}

	var c ShiftCipher
	c.distance = distance
	return c
}

type VigenereCipher struct {
	key string
}

func vigenereCode(str string, key string, isEncode bool) string {
	str = format(str)
	cipher := ""
	for i, s := range str {
		v := s
		r := rune(key[i%len(key)] - 'a')
		if isEncode {
			v += r
		} else {
			v -= r
		}
		if v-'a' >= 26 {
			v -= 26
		} else if v-'a' < 0 {
			v += 26
		}
		cipher += string(v)
	}
	return cipher
}

func (c VigenereCipher) Encode(str string) string {
	return vigenereCode(str, c.key, true)
}

func (c VigenereCipher) Decode(str string) string {
	return vigenereCode(str, c.key, false)
}

func NewVigenere(key string) Cipher {
	reg := regexp.MustCompile(`[^a-z]`)
	if len(key) <= 2 || reg.MatchString(key) {
		return nil
	}

	var c VigenereCipher
	c.key = key
	return c
}
