with open('Day 1 - input', 'r') as f:
    floors = f.readline().strip()
    final_floor = 0
    for i in floors:
        final_floor += (1 if i == '(' else -1)
    print(final_floor)
