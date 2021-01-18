package flatten

// Flatten returns a single flattened list.
func Flatten(arr interface{}) []interface{} {
	var acc []interface{}
	acc = doFlatten(acc, arr)
	if len(acc) == 0 {
		acc = []interface{}{}
	}
	return acc
}

func doFlatten(acc []interface{}, arr interface{}) []interface{} {
	switch v := arr.(type) {
	case int:
		acc = append(acc, v)
	case []interface{}:
		for i := range v {
			acc = doFlatten(acc, v[i])
		}
	}
	return acc
}
