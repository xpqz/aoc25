from functools import cache

with open("../d/11") as file:
    graph = {}
    for line in file:
        if line.strip():
            parts = line.replace(":", "").split()
            graph[parts[0]] = parts[1:]


@cache
def count_paths(start, end):
    if start == end:
        return 1
    return sum(count_paths(nxt, end) for nxt in graph.get(start, []))


print(count_paths("you", "out"))

grid = [["svr", "fft", "dac", "out"], ["svr", "dac", "fft", "out"]]


def chain_paths(waypoints):
    result = 1
    for a, b in zip(waypoints, waypoints[1:]):
        result *= count_paths(a, b)
    return result


print(sum(chain_paths(row) for row in grid))
