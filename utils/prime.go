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

// (Unique) prime factors
type UniquePrimeFactors []int

func (p *UniquePrimeFactors) Value() int {
	result := 1
	for _, f := range *p {
		result *= f
	}
	return result
}

func (p *UniquePrimeFactors) Insert(f int) bool {
	if !IsPrime(f) {
		panic("cannot insert non-prime")
	}
	i := sort.SearchInts(*p, f)
	if i < len(*p) && (*p)[i] == f {
		return false
	}
	if i == len(*p) {
		newP := append(*p, f)
		p = &newP
	} else {
		newP := make(UniquePrimeFactors, 0, len(*p) + 1)
		newP = append(newP, (*p)[0:i]...)
		newP = append(newP, f)
		newP = append(newP, (*p)[i:]...)
		p = &newP	
	}
	return true
}

// Factorize returns the unique prime factors of the given number
func Factorize(n int) UniquePrimeFactors {
	result := UniquePrimeFactors{}
	for p := 2; n > 1; p = nextPrimeAfter(p) {
		f := 0
		for ; n%p == 0; n /= p {
			f++
		}
		if f > 0 {
			result = append(result, p)
		}
	}
	return result
}
