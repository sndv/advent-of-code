import os

with open(os.path.join(os.path.dirname(__file__), "input")) as fd:
    puzzle_input = fd.read()


trees = list(list(map(int, list(ln))) for ln in puzzle_input.splitlines())
C = len(trees[0])
R = len(trees)

part1 = 0
part2 = -1

for r, row in enumerate(trees):
    for c, tr in enumerate(row):
        score = 1
        visible = False
        for rm, cm in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            curr_r = r
            curr_c = c
            dir_score = 0
            while True:
                curr_r += rm
                curr_c += cm
                if curr_r < 0 or curr_c < 0 or curr_r >= R or curr_c >= C:
                    if not visible:
                        part1 += 1
                        visible = True
                    break
                dir_score += 1
                if trees[curr_r][curr_c] >= trees[r][c]:
                    break
            score *= dir_score
        if score > part2:
            part2 = score


print(part1)
print(part2)
