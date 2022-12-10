import os

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    input = fd.read()


def solve(rope_len):
    rope = [[0, 0] for _ in range(rope_len)]
    visited = set()
    ft = lambda n: 0 if n == 0 else n // abs(n)
    for line in input.splitlines():
        d, n = line.split()
        n = int(n)
        for _ in range(n):
            mod = {"U": [-1, 0], "D": [1, 0], "R": [0, 1], "L": [0, -1]}[d]
            rope[0][0] += mod[0]
            rope[0][1] += mod[1]
            for i, knot in enumerate(rope[1:]):
                rd = rope[i][0] - knot[0]
                cd = rope[i][1] - knot[1]
                if abs(rd) > 1 or abs(cd) > 1:
                    knot[0] += ft(rd)
                    knot[1] += ft(cd)
            visited.add(tuple(rope[-1]))
    return len(visited)


print(solve(2))
print(solve(10))
