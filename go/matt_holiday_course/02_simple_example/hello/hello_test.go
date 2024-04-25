package hello

import "testing"

func TestSayHello(t *testing.T) {
	subtests := []struct {
		items  []string
		result string
	}{
		{
			result: "Hello, world!",
		},
		{
			items:  []string{"Chase"},
			result: "Hello, Chase!",
		},
		{
			items:  []string{"Chase", "Becca", "David"},
			result: "Hello, Chase, Becca, David!",
		},
	}

	for _, st := range subtests {
		if s := Say(st.items); s != st.result {
			t.Errorf("wanted %s (%v), got %s", st.items, st.result, s)
		}
	}
}
