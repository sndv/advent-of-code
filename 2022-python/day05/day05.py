import os
import re

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    puzzle_input = fd.read()

pos_str, steps_str = puzzle_input.split("\n\n")
pos_lines = pos_str.splitlines()
col_num = len(pos_lines[-1].split())
steps = steps_str.splitlines()

pos = [[] for _ in range(col_num)]

for step in pos_lines[:-1]:
    for c in range(col_num):
        if step[c * 4 + 1] != " ":
            pos[c].insert(0, step[c * 4 + 1])

pos2 = [el[:] for el in pos]

for step in steps:
    num, frm, to = (int(n) for n in re.findall(r"\d+", step))
    for _ in range(num):
        pos[to - 1].append(pos[frm - 1].pop())
    pos2[frm - 1], pos2[to - 1] = pos2[frm - 1][:-num], pos2[to - 1] + pos2[frm - 1][-num:]


part1 = "".join(el[-1] for el in pos)
part2 = "".join(el[-1] for el in pos2)

print(part1)
print(part2)
