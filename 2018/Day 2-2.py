from itertools import combinations

def hamming_dist(str1, str2):
    dist = 0
    for i, j in zip(str1, str2):
        if i != j:
            dist += 1
    return dist

with open("Day 2 - input", "r") as f:
    ids = [line.strip() for line in f]
    for id1, id2 in combinations(ids, 2):
        if hamming_dist(id1, id2) == 1:
            print(*(i for i, j in zip(id1, id2) if i == j), sep='')
