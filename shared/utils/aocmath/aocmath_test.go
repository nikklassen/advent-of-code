package aocmath

import "testing"

func TestLCMAll(t *testing.T) {
	if got := LCMAll([]uint{75, 100}); got != 300 {
		t.Errorf("got %d, want %d", got, 300)
	}
}

func FuzzPrimeFactors(f *testing.F) {
	f.Fuzz(func(t *testing.T, i uint) {
		if i <= 2 {
			t.Skip("out of range")
		}
		factors := PrimeFactors(i)
		prod := uint(1)
		for _, f := range factors {
			prod *= f
		}
		if prod != i {
			t.Errorf("PrimeFactors got %v, product %d, want %d", factors, prod, i)
		}
	})
}

func FuzzLCM(f *testing.F) {
	f.Fuzz(func(t *testing.T, x, y uint) {
		if x == 0 || y == 0 {
			t.Skip("out of range")
		}
		lcm := LCM(x, y)
		if lcm%x != 0 {
			t.Errorf("LCM(%d, %d) got %d, but %d is not factor", x, y, lcm, x)
		}
		if lcm%y != 0 {
			t.Errorf("LCM(%d, %d) got %d, but %d is not factor", x, y, lcm, y)
		}
	})
}

func FuzzGCD(f *testing.F) {
	f.Fuzz(func(t *testing.T, x, y uint) {
		if x == 0 || y == 0 {
			t.Skip("out of range")
		}
		gcd := GCD(x, y)
		if x%gcd != 0 {
			t.Errorf("GCD(%d, %d) got %d, but %d is not a multiple", x, y, gcd, x)
		}
		if y%gcd != 0 {
			t.Errorf("GCD(%d, %d) got %d, but %d is not a multiple", x, y, gcd, y)
		}
	})
}
