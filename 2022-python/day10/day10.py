import os

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    input = fd.read()

input = "\n\n" + input.replace("addx", "\naddx")

reg = 1
part1 = 0
part2 = ""

for i, line in enumerate(input.splitlines()):
    if line.startswith("addx"):
        reg += int(line.split()[1])
    if i % 40 == 20:
        part1 += i * reg
    if i % 40 == 0:
        part2 += "\n"
    part2 += "#" if (i - 1) % 40 in (reg - 1, reg, reg + 1) else " "


print(part1)
print(part2)
