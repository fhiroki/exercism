package strand

// ToRNA returns RNA complement.
func ToRNA(dna string) string {
	var rna string

	dnaToRna := map[rune]rune{
		'G': 'C',
		'C': 'G',
		'T': 'A',
		'A': 'U',
	}

	for _, d := range dna {
		rna += string(dnaToRna[d])
	}
	return rna
}
