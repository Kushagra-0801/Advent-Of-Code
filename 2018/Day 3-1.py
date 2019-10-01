canvas = {}
overlap = 0
with open('Day 3 - input') as f:
    for line in f:
        line = line.strip().split()
        leftxtop = line[2].split(',')
        left_margin = int(leftxtop[0])
        top_margin = int(leftxtop[1][:-1])
        widthxheight = line[3].split('x')
        width = int(widthxheight[0])
        height = int(widthxheight[1])
        for x in range(width):
            for y in range(height):
                try:
                    canvas[(left_margin + x, top_margin + y)] += 1
                    overlap += 1 if canvas[(left_margin + x, top_margin + y)] == 2 else 0
                except KeyError:
                    canvas[(left_margin + x, top_margin + y)] = 1
print(overlap)