with open('Day 8 - input', 'r') as f:
    result = 0
    for line in f:
        line = line.strip()
        result += 4 + line.count('\\') + line.count('"') - 2
    print(result)