package kindergarten

import (
	"errors"
	"sort"
	"strings"
)

type Garden struct {
	plants map[string][]string
}

func (g *Garden) Plants(child string) ([]string, bool) {
	plants, ok := g.plants[child]
	if !ok {
		return nil, false
	}
	return plants, true
}

func NewGarden(diagram string, childrenArg []string) (*Garden, error) {
	children := make([]string, len(childrenArg))
	copy(children, childrenArg)
	sort.Strings(children)

	for i := 1; i < len(children); i++ {
		if children[i-1] == children[i] {
			return nil, errors.New("duplicate name")
		}
	}

	seeds := map[rune]string{
		'G': "grass",
		'C': "clover",
		'R': "radishes",
		'V': "violets",
	}

	plants := make(map[string][]string, 0)
	diagrams := strings.Split(diagram, "\n")
	for i, d := range diagrams {
		if i == 0 {
			if len(d) != 0 {
				return nil, errors.New("wrong diagram format")
			}
			continue
		}

		if len(d)%2 != 0 {
			return nil, errors.New("odd number of cups")
		}

		for j := 0; j < len(d)/2; j++ {
			if len(children) <= j {
				return nil, errors.New("mismatched rows")
			}

			seed1 := seeds[rune(d[j*2])]
			seed2 := seeds[rune(d[j*2+1])]
			if len(seed1) == 0 || len(seed2) == 0 {
				return nil, errors.New("invalid cup codes")
			}

			plants[children[j]] = append(plants[children[j]], []string{seed1, seed2}...)
		}
	}

	var g Garden
	g.plants = plants

	return &g, nil
}
