import os
from itertools import product

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    puzzle_input = fd.read()


def neighbours(cube):
    for nm in [(0, 0, 1), (0, 0, -1), (0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0)]:
        yield tuple(sum(pair) for pair in zip(cube, nm))


def surface_area(cubes_list):
    exposed = 0
    for cube in cubes_list:
        exposed += 6 - sum(1 for n in neighbours(cube) if n in cubes_list)
    return exposed


cubes = set(tuple(map(int, line.split(","))) for line in puzzle_input.splitlines())
max_x, max_y, max_z = (max(c[i] for c in cubes) for i in range(3))
inner = set()

for cube in product(range(1, max_x), range(1, max_y), range(1, max_z)):
    if cube in cubes:
        continue
    for dir_cubes in [
        ((x, cube[1], cube[2]) for x in range(0, cube[0])),
        ((x, cube[1], cube[2]) for x in range(cube[0] + 1, max_x + 1)),
        ((cube[0], y, cube[2]) for y in range(0, cube[1])),
        ((cube[0], y, cube[2]) for y in range(cube[1] + 1, max_y + 1)),
        ((cube[0], cube[1], z) for z in range(0, cube[2])),
        ((cube[0], cube[1], z) for z in range(cube[2] + 1, max_z + 1)),
    ]:
        if not any(c in cubes for c in dir_cubes):
            break
    else:
        inner.add(cube)

while True:
    not_inner = set()
    all_cubes = cubes | inner
    inner = set(filter(lambda cube: all(n in all_cubes for n in neighbours(cube)), inner))
    if len(cubes) + len(inner) == len(all_cubes):
        break

part1 = surface_area(cubes)
part2 = part1 - surface_area(inner)

print(part1)
print(part2)
