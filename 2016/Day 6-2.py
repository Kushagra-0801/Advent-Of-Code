from collections import Counter

def extractFile(file):
    return [line for line in file]

with open('Day 6-Input') as file:
    lines = extractFile(file)
    lines = [[char for char in line[:-1]] for line in lines]
    lines = [Counter(line).most_common()[-1][0] for line in list(zip(*lines))]
    print(''.join(lines))
