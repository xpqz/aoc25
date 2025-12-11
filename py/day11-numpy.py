import numpy as np
import re

with open("../d/11") as file:
    f = [re.split(r"[ :]+", line.strip()) for line in file if line.strip()]

n = list(dict.fromkeys(node for line in f for node in line))

g = np.zeros((len(n), len(n)), dtype=int)
for line in f:
    src = n.index(line[0])
    for tgt in line[1:]:
        g[src, n.index(tgt)] = 1


def paths(pair):
    mid = len(pair) // 2
    start, end = n.index(pair[:mid]), n.index(pair[mid:])

    m = g.copy()
    m[end, end] = 1

    prev = None
    while not np.array_equal(m, prev):
        prev = m
        m = m @ m

    return m[start, end]


print(paths("youout"))

grid = [["svrfft", "fftdac", "dacout"], ["svrdac", "dacfft", "fftout"]]

print(sum(np.prod([paths(p) for p in row]) for row in grid))
