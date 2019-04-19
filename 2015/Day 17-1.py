from itertools import combinations

with open('Day 17 - input', 'r') as f:
    capacities = []
    total_eggnog = int(f.readline().strip().split()[-1])
    for line in f:
        line = line.strip().split()
        capacities.append(int(line[0]))
    total_ways = 0
    for i in range(1, len(capacities)+1):
        for j in combinations(capacities, i):
            total_ways += int(sum(j) == total_eggnog)
    print(total_ways)