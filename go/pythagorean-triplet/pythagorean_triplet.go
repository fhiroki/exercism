package pythagorean

type Triplet [3]int

func Range(min, max int) []Triplet {
	var ts []Triplet
	for a := min; a <= max; a++ {
		for b := a + 1; b <= max; b++ {
			for c := b + 1; c <= max; c++ {
				if a*a+b*b == c*c {
					ts = append(ts, Triplet{a, b, c})
				}
			}
		}
	}
	return ts
}

func Sum(p int) []Triplet {
	var ts []Triplet
	for a := 1; a <= p; a++ {
		for b := a + 1; b <= p; b++ {
			c := p - (a + b)
			if (c > 0) && (a*a+b*b == c*c) {
				ts = append(ts, Triplet{a, b, c})
			}
		}
	}
	return ts
}
