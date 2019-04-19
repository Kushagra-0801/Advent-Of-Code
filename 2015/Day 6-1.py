with open('Day 6 - input', 'r') as f:
    lights = [[False for _ in range(1000)] for _ in range(1000)]
    for instruction in f:
        instruction = instruction.split()
        if instruction[0] == 'turn':
            if instruction[1] == 'on':
                top_left = tuple(int(i) for i in instruction[2].split(','))
                bottom_right = tuple(int(i) for i in instruction[4].split(','))
                for i in range(top_left[0], bottom_right[0]+1):
                    for j in range(top_left[1], bottom_right[1]+1):
                        lights[i][j] = True
            else:
                top_left = tuple(int(i) for i in instruction[2].split(','))
                bottom_right = tuple(int(i) for i in instruction[4].split(','))
                for i in range(top_left[0], bottom_right[0]+1):
                    for j in range(top_left[1], bottom_right[1]+1):
                        lights[i][j] = False
        else:
            top_left = tuple(int(i) for i in instruction[1].split(','))
            bottom_right = tuple(int(i) for i in instruction[3].split(','))
            for i in range(top_left[0], bottom_right[0]+1):
                for j in range(top_left[1], bottom_right[1]+1):
                    lights[i][j] = (False if lights[i][j] else True)
    lights_on = 0
    for i in range(1000):
        for j in range(1000):
            lights_on += 1 if lights[i][j] else 0
    print(lights_on)
