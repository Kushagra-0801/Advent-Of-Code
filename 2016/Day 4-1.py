from collections import Counter
from functools import cmp_to_key

def extractFile(file):
    return [line[:-1] for line in file]

def sortComp(a, b): # parameters are tuples (letter, count)
    return ord(a[0]) - ord(b[0]) if a[1] == b[1] else b[1] - a[1] 

def mostCommon(line, n):
    letters = Counter(line).most_common()
    letters = [letter for letter in letters if letter[0].isalpha()]
    letters = sorted(letters, key=cmp_to_key(sortComp))
    return [letter for letter,_ in letters[:5]]

def validRoom(line):
    return "[{0}]".format(''.join(mostCommon(line[:-7], 5))) == line[-7:]

with open('Day 4-Input') as file:
    lines = extractFile(file)
    print(sum(int(line[-10:-7]) for line in lines if validRoom(line)))
