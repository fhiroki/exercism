package clock

import (
	"fmt"
)

// Clock is clock object
type Clock struct {
	h, m int
}

// Adjust returns adjusted time
func (c *Clock) Adjust() {
	for c.m < 0 {
		c.h--
		c.m += 60
	}
	for c.h < 0 {
		c.h += 24
	}
	if c.m >= 60 {
		c.h += c.m / 60
		c.m %= 60
	}
	if c.h >= 24 {
		c.h %= 24
	}
}

// New returns clock
func New(h, m int) Clock {
	c := Clock{h, m}
	c.Adjust()
	return c
}

// Add returns added time.
func (c Clock) Add(minute int) Clock {
	clock := Clock{c.h, c.m + minute}
	clock.Adjust()
	return clock
}

// Subtract returns subtracted time.
func (c Clock) Subtract(minute int) Clock {
	clock := Clock{c.h, c.m - minute}
	clock.Adjust()
	return clock
}

func (c Clock) String() string {
	return fmt.Sprintf("%02d:%02d", c.h, c.m)
}
