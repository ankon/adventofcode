package utils

import (
	"math"
	"sort"
)

var knownPrimes = []int{2,3,5,7,11,13,17,19,23}

func nextPrimeAfter(n int) int {
	i := sort.SearchInts(knownPrimes, n + 1)
	if i < len(knownPrimes) {
		return knownPrimes[i]
	}
	for x := knownPrimes[i - 1] + 2; ; x++ {
		if IsPrime(x) {
			if len(knownPrimes) < 1000 {
				knownPrimes = append(knownPrimes, x)
			}
			return x
		}
	}
}

func IsPrime(p int) bool {
	if p == 2 || p == 3 {
		return true
	}

	max := int(math.Ceil(math.Sqrt(float64(p))))
	for d := 2; d <= max; d = nextPrimeAfter(d) {
		if p%d == 0 {
			return false
		}
	}
	return true
}
