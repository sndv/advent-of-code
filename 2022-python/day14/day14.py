import os

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    puzzle_input = fd.read()

SOURCE = (500, 0)
blocked = set()

for line in puzzle_input.splitlines():
    rs = [list(map(int, p.split(","))) for p in line.split(" -> ")]
    for (x1, y1), (x2, y2) in zip(rs, rs[1:]):
        blocked |= set(
            (x, y)
            for x in range(min(x1, x2), max(x1, x2) + 1)
            for y in range(min(y1, y2), max(y1, y2) + 1)
        )

LP = max(el[1] for el in blocked)
part1 = part2 = 0
sp = SOURCE
while True:
    if part1 == 0 and sp[1] >= LP:
        part1 = part2
    x, y = new_sp = sp
    for nx, ny in ((x, y + 1), (x - 1, y + 1), (x + 1, y + 1)):
        if (nx, ny) not in blocked and ny < LP + 2:
            new_sp = (nx, ny)
            break
    if sp == new_sp:
        blocked.add(sp)
        part2 += 1
        if new_sp == SOURCE:
            break
        sp = SOURCE
        continue
    sp = new_sp

print(part1)
print(part2)
