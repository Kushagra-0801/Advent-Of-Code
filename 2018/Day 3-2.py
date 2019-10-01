from collections import namedtuple
from itertools import product

Claim = namedtuple('Claim', 'claim_id left_margin top_margin width height')
canvas = {}
claims = []
with open('Day 3 - input') as f:
    for line in f:
        line = line.strip().split()
        id_ = int(line[0][1:])
        leftxtop = line[2].split(',')
        left_margin = int(leftxtop[0])
        top_margin = int(leftxtop[1][:-1])
        widthxheight = line[3].split('x')
        width = int(widthxheight[0])
        height = int(widthxheight[1])
        claims.append(Claim(id_, left_margin, top_margin, width, height))
        for x in range(width):
            for y in range(height):
                try:
                    canvas[(left_margin + x, top_margin + y)] += 1
                except KeyError:
                    canvas[(left_margin + x, top_margin + y)] = 1
for claim in claims:
    for i, j in product(
        range(claim.left_margin, claim.left_margin + claim.width),
        range(claim.top_margin, claim.top_margin + claim.height)):
            if canvas[(i, j)] != 1:
                break
    else:
        print(claim.claim_id)
        break