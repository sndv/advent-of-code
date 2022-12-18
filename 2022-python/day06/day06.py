import os

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    puzzle_input = fd.read().strip()


def solve(gs):
    for i in range(len(puzzle_input)):
        if len(set(puzzle_input[i : i + gs])) == gs:
            return i + gs


print(solve(4))
print(solve(14))
