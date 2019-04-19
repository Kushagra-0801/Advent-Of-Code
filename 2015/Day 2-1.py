total_required = 0

with open('Day 2 - input', 'r') as f:
    for line in f:
        l, b, h = (int(i) for i in line.strip().split('x'))
        area1, area2, area3 = l*b, b*h, h*l
        lowest_area = min((area1, area2, area3))
        total_required += 2 * (area1 + area2 + area3) + lowest_area
print(total_required)
        