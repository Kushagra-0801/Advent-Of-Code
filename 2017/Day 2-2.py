checksum = 0
with open("Day 2 - input", "r") as file:
    for line in file:
        row = [int(i) for i in line.split('\t')]
        for i in range(0,len(row)):
            for second in row[i+1:]:
                first = row[i]
                result = 0
                if  (first/second).is_integer():
                    result = first/second
                elif (second/first).is_integer():
                    result = second/first
                checksum = checksum + result

print("The checksum is {}".format(checksum))
