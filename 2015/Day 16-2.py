from collections import namedtuple

with open('Day 16 - input', 'r') as f:
    Aunt = namedtuple(
        'Aunt', 'children cats samoyeds pomeranians akitas vizslas goldfish trees cars perfumes', defaults=[None, None, None, None, None, None, None, None, None, None])
    aunts = []
    for _, line in zip(range(500), f):
        line = line.strip().split()
        line = line[2:]
        for i, v in enumerate(line):
            line[i] = v.split(':')[0]
            line[i] = line[i].split(',')[0]
        aunts.append(
            eval(f'Aunt({line[0]}={line[1]}, {line[2]}={line[3]}, {line[4]}={line[5]})'))
    sender_aunt = Aunt(children=int(f.readline().strip().split(': ')[-1]),
                       cats=int(f.readline().strip().split(': ')[-1]),
                       samoyeds=int(f.readline().strip().split(': ')[-1]),
                       pomeranians=int(f.readline().strip().split(': ')[-1]),
                       akitas=int(f.readline().strip().split(': ')[-1]),
                       vizslas=int(f.readline().strip().split(': ')[-1]),
                       goldfish=int(f.readline().strip().split(': ')[-1]),
                       trees=int(f.readline().strip().split(': ')[-1]),
                       cars=int(f.readline().strip().split(': ')[-1]),
                       perfumes=int(f.readline().strip().split(': ')[-1])
                       )
    for i, a in enumerate(aunts):
        if a.children is None or a.children == sender_aunt.children:
            if a.cats is None or a.cats > sender_aunt.cats:
                if a.samoyeds is None or a.samoyeds == sender_aunt.samoyeds:
                    if a.pomeranians is None or a.pomeranians < sender_aunt.pomeranians:
                        if a.akitas is None or a.akitas == sender_aunt.akitas:
                            if a.vizslas is None or a.vizslas == sender_aunt.vizslas:
                                if a.goldfish is None or a.goldfish < sender_aunt.goldfish:
                                    if a.trees is None or a.trees > sender_aunt.trees:
                                        if a.cars is None or a.cars == sender_aunt.cars:
                                            if a.perfumes is None or a.perfumes == sender_aunt.perfumes:
                                                print(i+1)
