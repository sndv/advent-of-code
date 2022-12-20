import os

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    puzzle_input = fd.read()

numbers = [int(line) for line in puzzle_input.splitlines()]


def mix(nums, times=1):
    indexes = list(range(len(nums)))
    for _ in range(times):
        for i, n in enumerate(nums):
            old_pos = indexes.index(i)
            new_pos = (old_pos + n) % (len(nums) - 1)
            indexes.insert(new_pos, indexes.pop(old_pos))
    zero_idx = indexes.index(nums.index(0))
    return sum(nums[indexes[(zero_idx + n) % len(indexes)]] for n in (1000, 2000, 3000))


print(mix(numbers))
print(mix([n * 811589153 for n in numbers], 10))
