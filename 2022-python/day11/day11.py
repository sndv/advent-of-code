import os
import re
from math import prod
from types import SimpleNamespace as SN

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    puzzle_input = fd.read()


def parse_input():
    extract_nums = lambda s: list(map(int, re.findall(r"\d+", s)))
    monkeys = []
    for monkey_lines in puzzle_input.split("\n\n"):
        monkey_lines = monkey_lines.splitlines()
        monkey = SN()
        monkey.items = extract_nums(monkey_lines[1])
        monkey.op = monkey_lines[2].split()[-2:]
        monkey.test = extract_nums(monkey_lines[3])[0]
        monkey.true = extract_nums(monkey_lines[4])[0]
        monkey.false = extract_nums(monkey_lines[5])[0]
        monkeys.append(monkey)
    return monkeys


def solve(n, div):
    monkeys = parse_input()
    cm = prod(m.test for m in monkeys)
    insp_times = [0 for _ in monkeys]
    for _ in range(n):
        for i, m in enumerate(monkeys):
            insp_times[i] += len(m.items)
            for item in m.items:
                match m.op:
                    case ["*", "old"]:
                        item **= 2
                    case ["*", k]:
                        item *= int(k)
                    case ["+", k]:
                        item += int(k)
                item = (item // div) % cm
                idx = m.true if item % m.test == 0 else m.false
                monkeys[idx].items.append(item)
            m.items = []
    return prod(sorted(insp_times)[-2:])


print(solve(20, 3))
print(solve(10000, 1))
