package house

import "fmt"

type word struct {
	verb   string
	object string
}

func Verse(n int) string {
	words := []word{
		{"lay in", "house that Jack built"},
		{"ate", "malt"},
		{"killed", "rat"},
		{"worried", "cat"},
		{"tossed", "dog"},
		{"milked", "cow with the crumpled horn"},
		{"kissed", "maiden all forlorn"},
		{"married", "man all tattered and torn"},
		{"woke", "priest all shaven and shorn"},
		{"kept", "rooster that crowed in the morn"},
		{"belonged to", "farmer sowing his corn"},
		{"", "horse and the hound and the horn"},
	}

	verse := fmt.Sprintf("This is the %s", words[n-1].object)
	if n != 1 {
		verse += "\n"
	}

	for i := n - 2; i >= 0; i-- {
		verse += fmt.Sprintf("that %s the %s", words[i].verb, words[i].object)
		if i != 0 {
			verse += "\n"
		}
	}
	verse += "."

	return verse
}

func Song() string {
	var verses string
	for i := 1; i <= 12; i++ {
		verses += Verse(i)
		if i != 12 {
			verses += "\n\n"
		}
	}
	return verses
}
