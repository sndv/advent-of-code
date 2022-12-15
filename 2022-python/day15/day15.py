import os
import re

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    puzzle_input = fd.read()

TARGET_Y = 2_000_000
MAX_COORD = 4_000_000


def calc_dist(sx, sy, bx, by):
    return [sx, sy, bx, by, abs(sx - bx) + abs(sy - by)]


parsed = [calc_dist(*map(int, re.findall(r"-?\d+", line))) for line in puzzle_input.splitlines()]


def part1():
    beacons = set()
    scanned = set()
    for sx, sy, bx, by, dist in parsed:
        if by == TARGET_Y:
            beacons.add(bx)
        local_dist = dist - abs(sy - TARGET_Y)
        if local_dist >= 0:
            scanned |= set(range(sx - local_dist, sx + local_dist + 1))
    return len(scanned) - len(beacons)


def part2():
    for curr_y in range(MAX_COORD, 0, -1):
        scanned = [
            (sx - local_dist, sx + local_dist)
            for sx, sy, _, _, dist in parsed
            if (local_dist := dist - abs(sy - curr_y)) >= 0
        ]
        possible = set(p for s, e in scanned for p in (s - 1, e + 1) if 0 <= p <= MAX_COORD)
        for p in possible:
            for s, e in scanned:
                if s <= p <= e:
                    break
            else:
                return 4_000_000 * p + curr_y


print(part1())
print(part2())
