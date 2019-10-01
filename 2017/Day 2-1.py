
checksum = 0
with open("Day 2 - input", "r") as file:
    for line in file:
        row = [int(i) for i in line.split('\t')]
        largest = max(row)
        smallest = min(row)
        result = largest - smallest
        checksum = checksum + result
print("The checksum is {}".format(checksum))
