package utils

// MultiplyMod multiplies two numbers modulo m
//
// Taken from https://en.wikipedia.org/wiki/Modular_arithmetic#Example_implementations
func MultiplyMod(a, b, m uint64) uint64 {
	if ((a | b) & (0xFFFFFFFF << 32)) == 0 {
		return a * b % m
	}

	d := uint64(0)
	mp2 := m >> 1
	if a >= m {
		a %= m
	}
	if b >= m {
		b %= m
	}
	for i := 0; i < 64; i++ {
		if d > mp2 {
			d = (d << 1) - m
		} else {
			d <<= 1
		}
		if a&0x8000000000000000 != 0 {
			d += b
		}
		if d >= m {
			d -= m
		}
		a <<= 1
	}
	return d
}
