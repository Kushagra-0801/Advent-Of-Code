from itertools import combinations

with open('Day 17 - input', 'r') as f:
    capacities = []
    total_eggnog = int(f.readline().strip().split()[-1])
    for line in f:
        line = line.strip().split()
        capacities.append(int(line[0]))
    total_ways = 0
    for i in range(1, len(capacities)+1):
        got_here = False
        for j in combinations(capacities, i):
            if sum(j) == total_eggnog:
                if got_here:
                    total_ways += 1
                else:
                    got_here = True
                    total_ways = 1
        if got_here:
            print(total_ways)
            break

