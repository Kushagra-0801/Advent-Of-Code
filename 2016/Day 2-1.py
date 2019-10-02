sides = {"L": [0, -1], "U": [-1, 0], "R": [0, 1], "D": [1, 0]}
keypad = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
with open("Day 2 - input") as f:
    x, y = 1, 1
    code = ""
    for line in f:
        for direction in line.strip():
            x += sides[direction][0]
            y += sides[direction][1]
            x = min(max(x, 0), 2)
            y = min(max(y, 0), 2)
        code += str(keypad[x][y])
    print(code)
