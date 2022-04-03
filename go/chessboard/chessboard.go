package chessboard

// Declare a type named Rank which stores if a square is occupied by a piece - this will be a slice of bools
type Rank []bool

// Declare a type named Chessboard which contains a map of eight Ranks, accessed with keys from "A" to "H"
type Chessboard map[string]Rank

// CountInRank returns how many squares are occupied in the chessboard,
// within the given rank
func CountInRank(cb Chessboard, rank string) int {
	var sum int
	for _, occupied := range cb[rank] {
		if occupied {
			sum++
		}
	}
	return sum
}

// CountInFile returns how many squares are occupied in the chessboard,
// within the given file
func CountInFile(cb Chessboard, file int) int {
	if file < 1 || file > 8 {
		return 0
	}
	var sum int
	for _, rank := range cb {
		if rank[file-1] {
			sum++
		}
	}
	return sum
}

// CountAll should count how many squares are present in the chessboard
func CountAll(cb Chessboard) int {
	var sum int
	for _, rank := range cb {
		sum += len(rank)
	}
	return sum
}

// CountOccupied returns how many squares are occupied in the chessboard
func CountOccupied(cb Chessboard) int {
	var sum int
	for _, rank := range cb {
		for _, occupied := range rank {
			if occupied {
				sum++
			}
		}
	}
	return sum
}
