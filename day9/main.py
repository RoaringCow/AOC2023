def difference_list(values):
    diff_values = []
    for x in range(len(values) - 1):
        difference = int(values[x + 1]) - int(values[x])
        diff_values.append(difference)
    return diff_values

def all_zeros(values):
    for value in values:
        if value != 0:
            return False
    return True
        

with open("/home/ersan/AOC2023/day9/input.txt") as file:
    lines = file.readlines()

    sum1 = 0
    sum2 = 0
    for line in lines:
        lines_with_dif =[]
        values = line.strip().split(" ")
        lines_with_dif.append(values)
        while not all_zeros(lines_with_dif[len(lines_with_dif) - 1]):
            lines_with_dif.append(difference_list(lines_with_dif[len(lines_with_dif) - 1]))

        part2_lines = lines_with_dif

        lines_with_dif[-1].append(0)
        part2_lines[-1].insert(0, 0)
        for i in range(len(lines_with_dif) - 1, 0, -1):
            lines_with_dif[i - 1].append(lines_with_dif[i][-1] + int(lines_with_dif[i - 1][-1]))
            part2_lines[i - 1].insert(0, int(part2_lines[i - 1][0]) - part2_lines[i][0])
        sum1 += lines_with_dif[0][-1]
        sum2 += part2_lines[0][0]


        lines_with_dif[-1].insert(0, 0)

print(f"Part1: {sum1}, Part2: {sum2}")