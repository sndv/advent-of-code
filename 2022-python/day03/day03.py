import os

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    input = fd.read()

lines = [line.strip() for line in input.splitlines()]


def score(ch):
    return ord(ch) - ord("a") + 1 if ch.islower() else ord(ch) - ord("A") + 27


part1 = part2 = 0

for line in lines:
    for ch in set(line[: len(line) // 2]) & set(line[len(line) // 2 :]):
        part1 += score(ch)

for ln in range(0, len(lines), 3):
    ch = (set(lines[ln]) & set(lines[ln + 1]) & set(lines[ln + 2])).pop()
    part2 += score(ch)

print(part1)
print(part2)
