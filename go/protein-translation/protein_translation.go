package protein

import "errors"

var (
	// ErrStop is Stop sign
	ErrStop = errors.New("This code is STOP.")
	// ErrInvalidBase is Invalid code
	ErrInvalidBase = errors.New("This code is invalid.")
)

// FromCodon returns single codon.
func FromCodon(codon string) (string, error) {
	switch codon {
	case "AUG":
		return "Methionine", nil
	case "UUU", "UUC":
		return "Phenylalanine", nil
	case "UUA", "UUG":
		return "Leucine", nil
	case "UCU", "UCC", "UCA", "UCG":
		return "Serine", nil
	case "UAU", "UAC":
		return "Tyrosine", nil
	case "UGU", "UGC":
		return "Cysteine", nil
	case "UGG":
		return "Tryptophan", nil
	case "UAA", "UAG", "UGA":
		return "", ErrStop
	}
	return "", ErrInvalidBase
}

// FromRNA returns rna array.
func FromRNA(rna string) ([]string, error) {
	var codons []string
	for i := 0; i < len(rna); i += 3 {
		codon, err := FromCodon(rna[i : i+3])
		if err == ErrStop {
			break
		}
		if err == ErrInvalidBase {
			return codons, ErrInvalidBase
		}
		codons = append(codons, codon)
	}
	return codons, nil
}
