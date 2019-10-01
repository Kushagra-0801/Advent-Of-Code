from itertools import cycle
frequency = 0
reached_frequencies = {0}

with open('Day 1 - input', 'r') as f:
    for line in cycle(f):
        frequency += int(line.strip())
        if frequency not in reached_frequencies:
            reached_frequencies.add(frequency)
        else:
            break
print(frequency)
