package palindrome

import (
	"errors"
	"sort"
	"strconv"
)

type Product struct {
	Product        int      // palindromic, of course
	Factorizations [][2]int //list of all possible two-factor factorizations of Product, within given limits, in order
}

func isPalindrome(n int) bool {
	s := strconv.Itoa(n)
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		if s[i] != s[j] {
			return false
		}
	}
	return true
}

func Products(fmin, fmax int) (pmin, pmax Product, err error) {
	if fmin > fmax {
		return pmin, pmax, errors.New("fmin > fmax")
	}

	var products []Product

	for i := fmin; i <= fmax; i++ {
		for j := i; j <= fmax; j++ {
			p := i * j
			if isPalindrome(p) {
				product := Product{
					Product:        p,
					Factorizations: [][2]int{{i, j}},
				}
				products = append(products, product)
			}
		}
	}

	sort.Slice(products, func(i, j int) bool {
		return products[i].Product < products[j].Product
	})

	for _, p := range products {
		if p.Product == products[0].Product {
			pmin.Product = p.Product
			pmin.Factorizations = append(pmin.Factorizations, p.Factorizations[0])
		}
		if p.Product == products[len(products)-1].Product {
			pmax.Product = p.Product
			pmax.Factorizations = append(pmax.Factorizations, p.Factorizations[0])
		}
	}

	if pmin.Product == 0 && pmax.Product == 0 {
		return pmin, pmax, errors.New("no palindromes")
	}

	return pmin, pmax, nil
}
