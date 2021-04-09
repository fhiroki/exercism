package listops

type binFunc func(x, y int) int
type predFunc func(n int) bool
type unaryFunc func(x int) int
type IntList []int

func (list IntList) Foldr(fn binFunc, initial int) int {
	acc := initial
	for i := list.Length() - 1; i >= 0; i-- {
		acc = fn(list[i], acc)
	}
	return acc
}

func (list IntList) Foldl(fn binFunc, initial int) int {
	acc := initial
	for _, v := range list {
		acc = fn(acc, v)
	}
	return acc
}

func (list IntList) Filter(fn predFunc) IntList {
	want := make(IntList, 0)
	for _, v := range list {
		if fn(v) {
			want = append(want, v)
		}
	}
	return want
}

func (list IntList) Length() int {
	return len(list)
}

func (list IntList) Map(fn unaryFunc) IntList {
	want := make(IntList, 0)
	for _, v := range list {
		want = append(want, fn(v))
	}
	return want
}

func (list IntList) Reverse() IntList {
	want := make(IntList, len(list))
	for i, j := 0, list.Length()-1; i < j; i, j = i+1, j-1 {
		want[i], want[j] = list[j], list[i]
	}
	return want
}

func (list IntList) Append(other IntList) IntList {
	return append(list, other...)
}

func (list IntList) Concat(args []IntList) IntList {
	want := make(IntList, 0)
	want = append(want, list...)
	for _, other := range args {
		want = append(want, other...)
	}
	return want
}
