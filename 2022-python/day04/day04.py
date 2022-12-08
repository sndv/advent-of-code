import os
import re

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    input = fd.read()

part1 = part2 = 0

for line in input.strip().splitlines():
    a1, a2, b1, b2 = (int(e) for e in re.findall(r"\d+", line))
    if (a1 <= b1 and a2 >= b2) or (b1 <= a1 and b2 >= a2):
        part1 += 1
    if max(a1, b1) <= min(a2, b2):
        part2 += 1

print(part1)
print(part2)
