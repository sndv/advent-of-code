import os
from math import prod

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    puzzle_input = fd.read()


def compare(a, b):
    if isinstance(a, int):
        if isinstance(b, int):
            return a < b if a != b else None
        return compare([a], b)
    if isinstance(b, int):
        return compare(a, [b])
    for i in range(min(len(a), len(b))):
        res = compare(a[i], b[i])
        if res is not None:
            return res
    return len(a) < len(b) if len(a) != len(b) else None


class P:
    def __init__(self, value):
        self.value = eval(value)

    def __lt__(self, other):
        return compare(self.value, other.value)


part1 = 0

for i, lg in enumerate(puzzle_input.split("\n\n")):
    ln1, ln2 = lg.split("\n")
    if compare(eval(ln1), eval(ln2)):
        part1 += i + 1

print(part1)

dividers = [P("[[2]]"), P("[[6]]")]
packets = sorted([P(ln) for ln in puzzle_input.splitlines() if ln] + dividers)

part2 = prod(packets.index(d) + 1 for d in dividers)
print(part2)
