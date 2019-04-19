with open('Day 18 - input', 'r') as f:
    light_board = []
    STEPS = int(f.readline().strip().split()[-1])
    for line in f:
        line = line.strip()
        light_board.append(list(line))

    for _ in range(STEPS):
        new_board = [['.' for _ in range(100)] for _ in range(100)]
        for i in range(100):
            for j in range(100):
                count = 0
                if i == 0 and j == 0:
                    if light_board[0][1] == '#':
                        count += 1
                    if light_board[1][0] == '#':
                        count += 1
                    if light_board[1][1] == '#':
                        count += 1
                elif i == 99 and j == 0:
                    if light_board[99][1] == '#':
                        count += 1
                    if light_board[98][0] == '#':
                        count += 1
                    if light_board[98][1] == '#':
                        count += 1
                elif j == 0:
                    if light_board[i-1][0] == '#':
                        count += 1
                    if light_board[i-1][1] == '#':
                        count += 1
                    if light_board[i][1] == '#':
                        count += 1
                    if light_board[i+1][1] == '#':
                        count += 1
                    if light_board[i+1][0] == '#':
                        count += 1
                elif i == 0 and j == 99:
                    if light_board[i][98] == '#':
                        count += 1
                    if light_board[i+1][98] == '#':
                        count += 1
                    if light_board[i+1][99] == '#':
                        count += 1
                elif i == 99 and j == 99:
                    if light_board[i][98] == '#':
                        count += 1
                    if light_board[i-1][98] == '#':
                        count += 1
                    if light_board[i-1][99] == '#':
                        count += 1
                elif j == 99:
                    if light_board[i-1][99] == '#':
                        count += 1
                    if light_board[i-1][98] == '#':
                        count += 1
                    if light_board[i][98] == '#':
                        count += 1
                    if light_board[i+1][98] == '#':
                        count += 1
                    if light_board[i+1][99] == '#':
                        count += 1
                elif i == 0:
                    if light_board[0][j-1] == '#':
                        count += 1
                    if light_board[1][j-1] == '#':
                        count += 1
                    if light_board[1][j] == '#':
                        count += 1
                    if light_board[1][j+1] == '#':
                        count += 1
                    if light_board[0][j+1] == '#':
                        count += 1
                elif i == 99:
                    if light_board[99][j-1] == '#':
                        count += 1
                    if light_board[98][j-1] == '#':
                        count += 1
                    if light_board[98][j] == '#':
                        count += 1
                    if light_board[98][j+1] == '#':
                        count += 1
                    if light_board[99][j+1] == '#':
                        count += 1
                else:
                    if light_board[i-1][j-1] == '#':
                        count += 1
                    if light_board[i-1][j] == '#':
                        count += 1
                    if light_board[i-1][j+1] == '#':
                        count += 1
                    if light_board[i][j+1] == '#':
                        count += 1
                    if light_board[i+1][j+1] == '#':
                        count += 1
                    if light_board[i+1][j] == '#':
                        count += 1
                    if light_board[i+1][j-1] == '#':
                        count += 1
                    if light_board[i][j-1] == '#':
                        count += 1
                if light_board[i][j] == '#':
                    if count == 2 or count == 3:
                        new_board[i][j] = '#'
                else:
                    if count == 3:
                        new_board[i][j] = '#'
        light_board = new_board
    count = 0
    for i in range(100):
        for j in range(100):
            count += (1 if light_board[i][j] == '#' else 0)
    print(count)