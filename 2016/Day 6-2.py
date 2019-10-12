from collections import Counter

def extract_file(file):
    return [line.strip() for line in file]

with open('Day 6-Input') as file:
    lines = extract_file(file)
    lines = [Counter(line).most_common()[-1][0] for line in zip(*lines)]
    print(''.join(lines))