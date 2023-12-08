with open("/home/ersan/AOC2023/day8/input.txt", "r") as file:
    lines = file.readlines()

    elements = {line[0:3]: (line[7:10], line[12:15]) for line in lines[2:]}

    directions = lines[0]

    current_pos = "AAA"
    
    step_count = 0
    while current_pos != "ZZZ":
        for direction in directions[:-1]:
            print(current_pos, direction, step_count)
            if current_pos == "ZZZ":
                break
            if direction == "L":
                current_pos = elements[current_pos][0]
            else:
                current_pos = elements[current_pos][1]
            step_count += 1
    
    print(step_count)