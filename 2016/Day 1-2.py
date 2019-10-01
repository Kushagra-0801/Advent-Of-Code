sides = [[-1, 0], [0, -1], [1, 0], [0, 1]]
with open("Day 1 - input") as f:
    input_ = f.read()
    directions = input_.split(", ")
    facing = 0
    x_distance, y_distance = 0, 0
    visited = {(x_distance, y_distance)}
    for step in directions:
        turn, length = step[0], int(step[1:])
        facing = facing + 1 if turn == "L" else facing - 1
        for i in range(length):
            curr_visited = len(visited)
            x_distance += sides[facing % 4][0]
            y_distance += sides[facing % 4][1]
            visited.add((x_distance, y_distance))
            if len(visited) == curr_visited:
                print(abs(x_distance) + abs(y_distance))
                exit()
