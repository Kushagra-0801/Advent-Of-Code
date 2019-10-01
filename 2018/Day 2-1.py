from collections import Counter

twos = 0
threes = 0
with open("Day 2 - input", "r") as f:
    for line in f:
        id_ = Counter(line.strip())
        twos += 1 if 2 in id_.values() else 0
        threes += 1 if 3 in id_.values() else 0
print(twos * threes)
