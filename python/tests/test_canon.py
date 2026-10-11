from pathlib import Path
import random
import unittest

from mail_canon import InvalidEmail, canonicalize, normalize

ROOT = Path(__file__).resolve().parents[2]


class CanonTests(unittest.TestCase):
    def test_vectors(self):
        for row in (ROOT / "testdata/vectors.tsv").read_text().splitlines():
            if row.startswith("#"):
                continue
            name, encoded, domain, canonical, provider = row.split("\t")
            raw = bytes.fromhex(encoded).decode()
            with self.subTest(name=name):
                if domain == "-":
                    for function in (normalize, canonicalize):
                        with self.assertRaises(InvalidEmail):
                            function(raw)
                else:
                    result = normalize(raw)
                    self.assertEqual((result.domain, result.canonical, result.provider), (domain, canonical, provider))
                    self.assertEqual(canonicalize(raw), canonical)
                    self.assertEqual(canonicalize(canonical), canonical)

    def test_types(self):
        for value in (None, 1, True, b"u@example.com", [], {}):
            for function in (normalize, canonicalize):
                with self.assertRaises(TypeError):
                    function(value)
        for value in ("\ud800@example.com", "name \udfff <u@example.com>"):
            with self.assertRaises(InvalidEmail):
                normalize(value)

    def test_random_inputs_are_bounded_and_idempotent(self):
        randomizer = random.Random(82751)
        alphabet = "abcXYZ019._+-@<> %\t\r\n\x00é。\u200b"
        for _ in range(5000):
            raw = "".join(randomizer.choices(alphabet, k=randomizer.randrange(100)))
            try:
                result = normalize(raw)
            except InvalidEmail:
                continue
            self.assertTrue(result.canonical.isascii())
            self.assertEqual(canonicalize(result.canonical), result.canonical)


if __name__ == "__main__":
    unittest.main()
