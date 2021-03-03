package strain

// Ints defines a collection of int.
type Ints []int

// Strings defines a collection of string.
type Strings []string

// Lists defines a collection of ints.
type Lists [][]int

// Keep returns keep collections.
func (list Ints) Keep(f func(int) bool) Ints {
	var want Ints
	for _, v := range list {
		if f(v) {
			want = append(want, v)
		}
	}
	return want
}

// Discard returns discard collections.
func (list Ints) Discard(f func(int) bool) Ints {
	var want Ints
	for _, v := range list {
		if !f(v) {
			want = append(want, v)
		}
	}
	return want
}

// Keep returns keep collections.
func (list Strings) Keep(f func(string) bool) Strings {
	var want Strings
	for _, v := range list {
		if f(v) {
			want = append(want, v)
		}
	}
	return want
}

// Keep returns keep collections.
func (list Lists) Keep(f func([]int) bool) Lists {
	var want Lists
	for _, v := range list {
		if f(v) {
			want = append(want, v)
		}
	}
	return want
}
