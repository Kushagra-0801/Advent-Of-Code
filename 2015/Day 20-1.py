with open('Day 20 - input', 'r') as f:
    NUM_PRESENTS = int(f.readline().strip())
    NUM_ELVES = NUM_PRESENTS // 10
    houses = [10 for _ in range(NUM_ELVES)]
    for i in range(2, NUM_ELVES+1):
        for j in range(i-1, NUM_ELVES, i):
            houses[j] += i*10
    for i, v in enumerate(houses):
        if v > NUM_PRESENTS:
            print(i+1)
            break
