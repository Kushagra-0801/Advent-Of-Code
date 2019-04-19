from itertools import permutations
with open('Day 13 - input', 'r') as f:
    happiness_table = {}
    people = set()
    for line in f:
        line = line.strip().split()
        happiness_table[(line[0], line[-1][:-1])] = (int(line[3]) if line[2] == 'gain' else -int(line[3]))
        people.add(line[0])
    for person in people:
        happiness_table[(person, 'you')] = 0
        happiness_table[('you', person)] = 0
    people.add('you')
    max_happiness = 0
    n = len(people)
    for arrangement in permutations(people):
        current_happiness = 0
        for i in range(n):
            current_happiness += happiness_table[(arrangement[i], arrangement[(i+1) % n])] + happiness_table[(arrangement[(i+1) % n], arrangement[i])]
        if current_happiness > max_happiness:
            max_happiness = current_happiness
    print(max_happiness)
