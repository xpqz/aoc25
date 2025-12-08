import networkx as nx
from itertools import combinations
import math


def all_edges(points):
    distances = []
    for i, j in combinations(range(len(points)), 2):
        dist = math.sqrt(sum((a - b) ** 2 for a, b in zip(points[i], points[j])))
        distances.append((dist, i, j))
    distances.sort(key=lambda x: x[0])
    return distances


def build_graph(points, edges):
    G = nx.Graph()
    for i, point in enumerate(points):
        G.add_node(i, pos=point)
    for dist, i, j in edges:
        G.add_edge(i, j, weight=dist)

    return G


def find_largest(G, n=3):
    components = list(nx.connected_components(G))
    components.sort(key=len, reverse=True)
    return components[:n]


def read_data(path):
    points = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            coords = tuple(map(int, line.split(",")))
            points.append(coords)
    return points


def part2(points, edges=None):
    n = len(points)
    if n < 2:
        return None

    if edges is None:
        edges = all_edges(points)

    parent = list(range(n))
    rank = [0] * n
    components = n

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def union(a, b):
        nonlocal components
        ra, rb = find(a), find(b)
        if ra == rb:
            return False
        if rank[ra] < rank[rb]:
            parent[ra] = rb
        elif rank[ra] > rank[rb]:
            parent[rb] = ra
        else:
            parent[rb] = ra
            rank[ra] += 1
        components -= 1
        return True

    last_edge = None
    for dist, i, j in edges:
        merged = union(i, j)
        if merged:
            last_edge = (points[i], points[j], dist)
            if components == 1:
                break

    return last_edge


def main():
    points = read_data("../d/8")
    edges = all_edges(points)
    G = build_graph(points, edges[:1000])
    p1 = math.prod([len(component) for component in find_largest(G, n=3)])
    print(f"Part 1: {p1}")
    edge = part2(points, edges)
    if edge is None:
        raise AssertionError("dataset empty or single point.")
    start, end, _ = edge
    print(f"Part 2: {start[0] * end[0]}")


if __name__ == "__main__":
    main()
