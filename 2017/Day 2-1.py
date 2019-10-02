checksum = 0
with open("Day 2 - input", "r") as file:
    for line in file:
        row = [int(i) for i in line.split()]
        result = max(row) - min(row)
        checksum += result
print(f"The checksum is {checksum}")
