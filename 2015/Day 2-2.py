total_required = 0

with open('Day 2 - input', 'r') as f:
    for line in f:
        l, b, h = (int(i) for i in line.strip().split('x'))
        x = l+b+h
        min_semi_perimeter = x - max((l, b, h))
        total_required += 2*min_semi_perimeter + l*b*h
print(total_required)
