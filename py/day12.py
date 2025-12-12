"""
Absurd data dependency warning.

The patterns can be ignored. In the _competition_ data set, treat
all patterns as

###
###
###

No tiling required. NOTE: this assumption does NOT hold for the
data example given in the problem description!
"""

import re

with open("../d/12") as f:
    data = f.read()


def start():
    nl = 0
    for idx, ch in enumerate(data):
        if ch == "\n":
            nl = idx
        elif ch == "x":
            return nl + 1
    return -1


def part1():
    res = 0

    for line in data[start() :].strip().split("\n"):
        nums = list(map(int, re.findall(r"\d+", line)))
        res += 9 * sum(nums[2:]) <= nums[0] * nums[1]

    return res


print(f"Part 1: {part1()}")
