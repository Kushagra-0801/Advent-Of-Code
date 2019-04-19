with open('Day 8 - input', 'r') as f:
    result = 0
    for line in f:
        line = line.strip()
        result += len(line) - len(eval(line))
    print(result)
