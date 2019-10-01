sides = [[-1, 0], [0, -1], [1, 0], [0, 1]]
with open("Day 1 - input") as f:
    input_ = f.read()
    directions = input_.split(", ")
    facing = 0
    x_distance, y_distance = 0, 0
    for step in directions:
        turn, length = step[0], int(step[1:])
        facing = facing + 1 if turn == "L" else facing - 1
        x_distance += length * sides[facing % 4][0]
        y_distance += length * sides[facing % 4][1]
    print(abs(x_distance) + abs(y_distance))
