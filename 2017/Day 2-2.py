from itertools import combinations
checksum = 0
with open("Day 2 - input", "r") as file:
    for line in file:
        row = [int(i) for i in line.split('\t')]
        combs = list(combinations(row,2))
        for combination in combs:
            result = 0
            first = combination[0]/combination[1]
            second = combination[1]/combination[0]
            if  first.is_integer():
                result = first
            elif second.is_integer():
                result = second
            checksum = checksum + result
            if result != 0:
                break

print("The checksum is {}".format(checksum))
