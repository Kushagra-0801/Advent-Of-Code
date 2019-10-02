sides = {"L": [0, -1], "U": [-1, 0], "R": [0, 1], "D": [1, 0]}
keypad = [
    ["", "", "1", "", ""],
    ["", "2", "3", "4", ""],
    ["5", "6", "7", "8", "9"],
    ["", "A", "B", "C", ""],
    ["", "", "D", "", ""],
]
with open("Day 2 - input") as f:
    x, y = 2, 0
    code = ""
    for line in f:
        for direction in line.strip():
            prev_x, prev_y = x, y
            x += sides[direction][0]
            y += sides[direction][1]
            if x + y < 2 or y - x > 2 or x - y > 2 or x + y > 6:
                x, y = prev_x, prev_y
        code += str(keypad[x][y])
    print(code)
