import os
from collections import defaultdict

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    puzzle_input = fd.read()

path = []
sizes = defaultdict(int)

for line in puzzle_input.splitlines():
    if line.startswith("$"):
        if line.split()[1] == "cd":
            match line.split()[2]:
                case "/":
                    path = []
                case "..":
                    path.pop()
                case dirname:
                    path.append(dirname)
    elif not line.startswith("dir"):
        size = line.split()[0]
        for i in range(len(path) + 1):
            dir_path = "/".join(path[:i])
            sizes[dir_path] += int(size)

part1 = sum(size for size in sizes.values() if size <= 100000)
part2 = min(size for size in sizes.values() if size >= (sizes[""] - 40000000))

print(part1)
print(part2)
