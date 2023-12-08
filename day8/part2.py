import math

def find_gcd(x, y):
    while y:
        x, y = y, x % y
    return x

def find_lcm(numbers):
    if len(numbers) < 2:

    lcm = numbers[0]
    for i in range(1, len(numbers)):
        lcm = (lcm * numbers[i]) // find_gcd(lcm, numbers[i])

    return lcm



with open("/home/ersan/AOC2023/day8/input.txt", "r") as file:
    lines = file.readlines()

    elements = {line[0:3]: (line[7:10], line[12:15]) for line in lines[2:]}
    starting_pos = [line[0:3] for line in lines[2:] if line[0:3][2] == "A"]
    directions = lines[0]
    
    step_counts = []

    for current_pos in starting_pos:
    
        step_count = 0
        while current_pos[2] != "Z":
            for direction in directions[:-1]:
                if current_pos == "ZZZ":
                    break
                if direction == "L":
                    current_pos = elements[current_pos][0]
                else:
                    current_pos = elements[current_pos][1]
                step_count += 1
        print(step_count, current_pos)
        step_counts.append(step_count)

    print(find_lcm(step_counts))
    

