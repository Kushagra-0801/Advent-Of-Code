def validTriangle(sides):
    return sides[0] + sides[1] > sides[2] and sides[0] + sides[2] > sides[1] and sides[1] + sides[2] > sides[0]

def toIntArray(arr):
    return [int(entry) for entry in arr]

def extractFile(file):
    return [toIntArray(line.strip().split()) for line in file]

with open('Day 3-Input') as file:
    print sum(1 for sides in extractFile(file) if validTriangle(sides))
