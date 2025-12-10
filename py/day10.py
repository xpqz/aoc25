from collections import deque
import pulp


def read_data(path):
    data = []
    with open(path) as f:
        for line in f:
            fields = line.strip().split(" ")
            pattern = fields[0].replace(".", "0").replace("#", "1")[1:-1]
            trnf = [tuple(int(d) for d in t[1:-1].split(",")) for t in fields[1:-1]]
            jolt = tuple(int(j) for j in fields[-1][1:-1].split(","))
            data.append((int(pattern, 2), len(pattern), trnf, jolt))
    return data


def to_bitmask(positions, length):
    p = ["0"] * length
    for d in positions:
        p[d] = "1"
    return int("".join(p), 2)


def part1(start, end, patterns):
    target = start ^ end
    if target == 0:
        return 0

    visited = {target}
    queue = deque([(target, 0)])

    while queue:
        state, depth = queue.popleft()
        for p in patterns:
            next_state = state ^ p
            if next_state == 0:
                return depth + 1
            if next_state not in visited:
                visited.add(next_state)
                queue.append((next_state, depth + 1))

    return -1


def part2(end, patterns):
    n = len(end)
    m = len(patterns)

    prob = pulp.LpProblem("min_transforms", pulp.LpMinimize)
    x = [pulp.LpVariable(f"x_{j}", lowBound=0, cat="Integer") for j in range(m)]
    prob += pulp.lpSum(x)
    for i in range(n):
        prob += (
            pulp.lpSum(x[j] for j in range(m) if i in patterns[j]) == end[i],
            f"row_{i}",
        )

    _status = prob.solve(pulp.PULP_CBC_CMD(msg=False))
    return int(pulp.value(pulp.lpSum(x)))


if __name__ == "__main__":
    data = read_data("../d/10")

    sum = [0, 0]
    for pat, length, trf, jolt in data:
        patterns = [to_bitmask(t, length) for t in trf]
        sum[0] += part1(0, pat, patterns)
        sum[1] += part2(jolt, trf)
    print(sum)
