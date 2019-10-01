frequency = 0

with open("Day 1 - input", "r") as f:
    for line in f:
        frequency += int(line.strip())
print(frequency)
