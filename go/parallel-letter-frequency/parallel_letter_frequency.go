package letter

import "sync"

// FreqMap records the frequency of each rune in a given text.
type FreqMap map[rune]int

// Frequency counts the frequency of each rune in a given text and returns this
// data as a FreqMap.
func Frequency(s string) FreqMap {
	m := FreqMap{}
	for _, r := range s {
		m[r]++
	}
	return m
}

// ConcurrentFrequency returns letter frequency.
func ConcurrentFrequency(arr []string) FreqMap {
	m := FreqMap{}
	mutex := &sync.Mutex{}
	wg := &sync.WaitGroup{}

	for _, s := range arr {
		wg.Add(1)
		go func(s string) {
			defer wg.Done()
			for _, r := range s {
				mutex.Lock()
				m[r]++
				mutex.Unlock()
			}
		}(s)
	}
	wg.Wait()

	return m
}
