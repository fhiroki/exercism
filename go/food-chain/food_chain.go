package foodchain

import "fmt"

func Verse(v int) string {
	creatures := []string{"fly", "spider", "bird", "cat", "dog", "goat", "cow", "horse"}
	lyric := fmt.Sprintf("I know an old lady who swallowed a %s.\n", creatures[v-1])
	if v == 8 {
		lyric += "She's dead, of course!"
		return lyric
	}

	second_phrase := []string{
		"It wriggled and jiggled and tickled inside her.",
		"How absurd to swallow a bird!",
		"Imagine that, to swallow a cat!",
		"What a hog, to swallow a dog!",
		"Just opened her throat and swallowed a goat!",
		"I don't know how she swallowed a cow!",
	}
	if v >= 2 {
		lyric += second_phrase[v-2] + "\n"
	}

	for i := v - 1; i > 0; i-- {
		lyric += fmt.Sprintf("She swallowed the %s to catch the %s", creatures[i], creatures[i-1])
		if v != 2 && i == 2 {
			lyric += " that wriggled and jiggled and tickled inside her.\n"
		} else {
			lyric += ".\n"
		}
	}

	lyric += "I don't know why she swallowed the fly. Perhaps she'll die."
	return lyric
}

func Verses(from, to int) string {
	lyric := ""
	for i := from; i <= to; i++ {
		lyric += Verse(i)
		if i != to {
			lyric += "\n\n"
		}
	}
	return lyric
}

func Song() string {
	return Verses(1, 8)
}
