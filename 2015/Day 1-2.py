with open('Day 1 - input', 'r') as f:
    floors = f.readline().strip()
    final_floor = 0
    for i, v in enumerate(floors):
        final_floor += (1 if v == '(' else -1)
        if final_floor == -1:
            print(i + 1)
            break
