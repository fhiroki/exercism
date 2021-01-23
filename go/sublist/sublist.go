package sublist

// Relation between first list and second list.
type Relation string

func isSameArray(A, B []int) bool {
	if len(A) != len(B) {
		return false
	}
	for i := 0; i < len(A); i++ {
		if A[i] != B[i] {
			return false
		}
	}
	return true
}

// Sublist returns if the first list contained within the second list.
func Sublist(A, B []int) Relation {
	if len(A) == len(B) {
		if isSameArray(A, B) {
			return "equal"
		}
	} else if len(A) < len(B) {
		for i := 0; i <= len(B)-len(A); i++ {
			if isSameArray(A, B[i:i+len(A)]) {
				return "sublist"
			}
		}
	} else {
		for i := 0; i <= len(A)-len(B); i++ {
			if isSameArray(A[i:i+len(B)], B) {
				return "superlist"
			}
		}
	}
	return "unequal"
}
