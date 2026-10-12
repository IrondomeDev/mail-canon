package mailcanon

import (
	"encoding/hex"
	"os"
	"strings"
	"testing"
)

func TestVectors(t *testing.T) {
	testVectors(t, "vectors.tsv", Normalize, Canonicalize)
}

func TestStrictVectors(t *testing.T) {
	testVectors(t, "strict-vectors.tsv", NormalizeStrict, CanonicalizeStrict)
}

func testVectors(t *testing.T, file string, normalize func(string) (Identity, error), canonicalize func(string) (string, error)) {
	t.Helper()
	content, err := os.ReadFile("../testdata/" + file)
	if err != nil {
		t.Fatal(err)
	}
	for _, row := range strings.Split(strings.TrimSpace(string(content)), "\n") {
		if strings.HasPrefix(row, "#") {
			continue
		}
		fields := strings.Split(strings.TrimSuffix(row, "\r"), "\t")
		t.Run(fields[0], func(t *testing.T) {
			t.Parallel()
			bytes, err := hex.DecodeString(fields[1])
			if err != nil {
				t.Fatal(err)
			}
			raw := string(bytes)
			value, err := normalize(raw)
			if fields[2] == "-" {
				if err != ErrInvalidEmail {
					t.Fatalf("expected ErrInvalidEmail, got %v", err)
				}
				if _, err := canonicalize(raw); err != ErrInvalidEmail {
					t.Fatal(err)
				}
				return
			}
			if err != nil {
				t.Fatal(err)
			}
			expected := Identity{fields[2], fields[3], fields[4]}
			if value != expected {
				t.Fatalf("got %#v, want %#v", value, expected)
			}
			if s, err := canonicalize(raw); err != nil || s != value.Canonical {
				t.Fatalf("canonical: %q %v", s, err)
			}
			if s, err := canonicalize(value.Canonical); err != nil || s != value.Canonical {
				t.Fatalf("not idempotent: %q %v", s, err)
			}
		})
	}
}

func TestInvalidUTF8(t *testing.T) {
	for _, raw := range []string{"\xff@example.com", "name \xed\xa0\x80 <u@example.com>"} {
		if _, err := Normalize(raw); err != ErrInvalidEmail {
			t.Fatalf("got %v", err)
		}
		if _, err := NormalizeStrict(raw); err != ErrInvalidEmail {
			t.Fatalf("strict: got %v", err)
		}
	}
}

func FuzzNormalize(f *testing.F) {
	for _, s := range []string{"User.Name+tag@gmail.com", "u@bücher.example", "a.+tag@outlook.com", "", "u@example.com"} {
		f.Add(s)
	}
	f.Fuzz(func(t *testing.T, raw string) {
		value, err := Normalize(raw)
		if err != nil {
			if err != ErrInvalidEmail {
				t.Fatal(err)
			}
			return
		}
		if !ascii(value.Canonical) {
			t.Fatal("non-ASCII canonical output")
		}
		if next, err := Canonicalize(value.Canonical); err != nil || next != value.Canonical {
			t.Fatalf("not idempotent: %q %v", next, err)
		}
	})
}

var benchmarkIdentity Identity

func BenchmarkNormalize(b *testing.B) {
	for _, value := range []string{"user@example.com", "User.Name+tag@GoogleMail.com", "User@bücher.example"} {
		b.Run(value, func(b *testing.B) {
			b.ReportAllocs()
			for i := 0; i < b.N; i++ {
				result, err := Normalize(value)
				if err != nil {
					b.Fatal(err)
				}
				benchmarkIdentity = result
			}
		})
	}
}
