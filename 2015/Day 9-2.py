from itertools import permutations
with open('Day 9 - input', 'r') as f:
    distances = {}
    cities = set()
    highest_distance = 0
    for line in f:
        line = line.strip().split()
        distances[frozenset((line[0], line[2]))] = int(line[4])
        cities.add(line[0])
        cities.add(line[2])
    for route in permutations(cities):
        current_distance = 0
        r1, r2 = iter(route), iter(route)
        next(r2)
        for c1, c2 in zip(r1, r2):
            current_distance += distances[frozenset((c1, c2))]
        if current_distance > highest_distance:
            highest_distance = current_distance
    print(highest_distance)
