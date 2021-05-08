package summultiples

func SumMultiples(limit int, divisors ...int) int {
	var multiples []int
	keys := make(map[int]bool)

	for _, divisor := range divisors {
		i := 1
		for {
			num := divisor * i
			if num == 0 || num >= limit {
				break
			}
			if keys[num] {
				i++
				continue
			}
			multiples = append(multiples, num)
			keys[num] = true
			i++
		}
	}

	sum := 0
	for _, v := range multiples {
		sum += v
	}
	return sum
}
