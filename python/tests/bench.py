"""Warm per-call timings; run after installing the package."""

import json
import platform
import statistics
import timeit
from importlib.metadata import version

from mail_canon import normalize

CASES = {
    "ascii": "user@example.com",
    "gmail": "User.Name+tag@GoogleMail.com",
    "idn": "User@bücher.example",
}
ITERATIONS = 50_000
SAMPLES = 5
results = {}
for name, value in CASES.items():
    for _ in range(5_000):
        normalize(value)
    samples = timeit.repeat(lambda: normalize(value), number=ITERATIONS, repeat=SAMPLES)
    results[name] = {
        "median_ns_per_call": round(statistics.median(samples) * 1e9 / ITERATIONS),
        "sample_ns_per_call": [round(sample * 1e9 / ITERATIONS) for sample in samples],
    }
print(json.dumps({
    "python": platform.python_version(), "idna": version("idna"),
    "iterations_per_sample": ITERATIONS, "samples": SAMPLES,
    "includes_startup": False, "results": results,
}, indent=2))
