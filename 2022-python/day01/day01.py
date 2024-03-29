import os

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    puzzle_input = fd.read()

totals = sorted(sum(map(int, group.strip().split())) for group in puzzle_input.split("\n\n"))

print(totals[-1])
print(sum(totals[-3:]))
