package blackjack

// ParseCard returns the integer value of a card following blackjack ruleset.
func ParseCard(card string) int {
	switch card {
	case "ace":
		return 11
	case "two":
		return 2
	case "three":
		return 3
	case "four":
		return 4
	case "five":
		return 5
	case "six":
		return 6
	case "seven":
		return 7
	case "eight":
		return 8
	case "nine":
		return 9
	case "ten",
		"jack",
		"queen",
		"king":
		return 10
	default:
		return 0
	}
}

// FirstTurn returns the decision for the first turn, given two cards of the
// player and one card of the dealer.
func FirstTurn(card1, card2, dealerCard string) string {
	v1 := ParseCard(card1)
	v2 := ParseCard(card2)
	dealer := ParseCard(dealerCard)
	sum := v1 + v2
	switch {
	case v1 == 11 && v2 == 11:
		return "P"
	case sum == 21 && dealer < 10:
		return "W"
	case sum >= 17:
		return "S"
	case sum >= 12 && dealer < 7:
		return "S"
	default:
		return "H"
	}
}
