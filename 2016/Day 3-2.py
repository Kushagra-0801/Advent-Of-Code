import numpy as np

def validTriangle(sides):
    return sides[0] + sides[1] > sides[2] and sides[0] + sides[2] > sides[1] and sides[1] + sides[2] > sides[0]

def toIntArray(arr):
    return [int(entry) for entry in arr]

def extractFile(file, take):
    return [toIntArray(line.strip().split()) for line in file]

with open('Day 3-Input') as file:
    lines = extractFile(file, 3)
    arr = []
    for i in range(int(len(lines) / 3)):
        j = i * 3
        arr.append([lines[j][0], lines[j + 1][0], lines[j + 2][0]])
        arr.append([lines[j][1], lines[j + 1][1], lines[j + 2][1]])
        arr.append([lines[j][2], lines[j + 1][2], lines[j + 2][2]])
    print(sum(1 for sides in arr if validTriangle(sides)))