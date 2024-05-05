package main

func main() {

	// ARRAYS
	// all these are equivalent
	var a [3]int
	b := [3]int{0, 0, 0}
	c := [...]int{0, 0, 0} // sized by initializer

	var d [3]int
	d = b // elements copied

	m := [...]int{1, 2, 3, 4}

	c = m // type mismatch, different sizees

	// SLICES - like arrays, but unsized
	var a2 []int         // nil, no storage
	var b2 = []int{1, 2} // initialized, size 2

	a2 = append(a2, 1) // append to nil OK
	b2 = append(b2, 3) // []int{1, 2, 3}

	a2 = b2 // overwrites a

	d2 := make([]int, 5) // []int{0, 0, 0, 0, 0}
	e2 := a2             // same storage (alias)

	e2[0] == b2[0] // true
}
