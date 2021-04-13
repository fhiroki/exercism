package pascal

func Triangle(n int) [][]int {
	var triangle [][]int
	triangle = append(triangle, []int{1})

	for i := 1; i < n; i++ {
		var arr []int
		arr = append(arr, 1)
		t := triangle[i-1]
		for j := 0; j < len(t)-1; j++ {
			arr = append(arr, t[j]+t[j+1])
		}
		arr = append(arr, 1)
		triangle = append(triangle, arr)
	}

	return triangle
}
