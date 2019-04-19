with open('Day 6 - input', 'r') as f:
    lights = [[0 for _ in range(1000)] for _ in range(1000)]
    for instruction in f:
        instruction = instruction.split()
        if instruction[0] == 'turn':
            if instruction[1] == 'on':
                top_left = tuple(int(i) for i in instruction[2].split(','))
                bottom_right = tuple(int(i) for i in instruction[4].split(','))
                for i in range(top_left[0], bottom_right[0]+1):
                    for j in range(top_left[1], bottom_right[1]+1):
                        lights[i][j] += 1
            else:
                top_left = tuple(int(i) for i in instruction[2].split(','))
                bottom_right = tuple(int(i) for i in instruction[4].split(','))
                for i in range(top_left[0], bottom_right[0]+1):
                    for j in range(top_left[1], bottom_right[1]+1):
                        lights[i][j] += (-1 if lights[i][j] else 0)
        else:
            top_left = tuple(int(i) for i in instruction[1].split(','))
            bottom_right = tuple(int(i) for i in instruction[3].split(','))
            for i in range(top_left[0], bottom_right[0]+1):
                for j in range(top_left[1], bottom_right[1]+1):
                    lights[i][j] += 2
    brightness = 0
    for i in range(1000):
        for j in range(1000):
            brightness += lights[i][j]
    print(brightness)
