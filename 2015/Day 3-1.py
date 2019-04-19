houses = {(0, 0)}
current_location = [0, 0]

with open('Day 3 - input', 'r') as f:
    directions = f.readline()
    for direction in directions:
        if direction == '^':
            current_location[1] += 1
            houses.add(tuple(current_location))
        elif direction == '>':
            current_location[0] += 1
            houses.add(tuple(current_location))
        elif direction == 'v':
            current_location[1] -= 1
            houses.add(tuple(current_location))
        else:
            current_location[0] -= 1
            houses.add(tuple(current_location))
print(len(houses))
