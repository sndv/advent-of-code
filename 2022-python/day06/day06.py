import os

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    input = fd.read().strip()


def solve(gs):
    for i in range(len(input)):
        if len(set(input[i : i + gs])) == gs:
            return i + gs


print(solve(4))
print(solve(14))
