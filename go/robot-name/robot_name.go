package robotname

import (
	"fmt"
	"math/rand"
	"time"
)

var names = map[string]bool{}

// Robot type
type Robot struct {
	name string
}

// Name generates robot name.
func (r *Robot) Name() (string, error) {
	if r.name != "" {
		return r.name, nil
	}
	rand.Seed(time.Now().UnixNano())
	for {
		r.name = fmt.Sprintf("%c%c%03d", 'A'+rune(rand.Intn(26)), 'A'+rune(rand.Intn(26)), rand.Intn(1000))
		if !names[r.name] {
			names[r.name] = true
			return r.name, nil
		}
	}
}

// Reset robot name.
func (r *Robot) Reset() {
	r.name = ""
}
