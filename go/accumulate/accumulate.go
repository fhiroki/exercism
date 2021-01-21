package accumulate

// Accumulate returns a new collection.
func Accumulate(strings []string, converter func(string) string) []string {
	output := make([]string, len(strings))
	for i, s := range strings {
		output[i] = converter(s)
	}
	return output
}
