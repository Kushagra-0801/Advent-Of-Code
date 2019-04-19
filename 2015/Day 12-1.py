import re
with open('Day 12 - input', 'r') as f:
    print(sum((int(i) for i in re.findall(r'-?\d+', f.readline().strip()))))
