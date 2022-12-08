import os

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    input = fd.read()

lines = [line.strip() for line in input.splitlines()]

part1 = part2 = 0

for line in lines:
    a, b = "_ABC".index(line[0]), "_XYZ".index(line[2])
    part1 += b
    if a == b:
        part1 += 3
    elif (b - a) % 3 == 1:
        part1 += 6
    part2 += [0, 0, 3, 6][b]
    part2 += (b + a) % 3 + 1


print(part1)
print(part2)
