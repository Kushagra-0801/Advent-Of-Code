from itertools import combinations
checksum = 0
with open("Day 2 - input", "r") as file:
    for line in file:
        row = [int(i) for i in line.split('\t')]
        for i,j in combinations(row,2):
            result = 0
            first = i/j
            second = j/i
            if  first.is_integer():
                result = first
            elif second.is_integer():
                result = second
            checksum = checksum + result
            if result != 0:
                break

print(f"The checksum is {checksum}")
