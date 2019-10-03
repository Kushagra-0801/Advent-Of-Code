from collections import Counter
from functools import cmp_to_key

def extract_file(file):
    return [line.strip() for line in file]

def sort_comp(a, b): # parameters are tuples (letter, count)
    return ord(a[0]) - ord(b[0]) if a[1] == b[1] else b[1] - a[1] 

def most_common(line, n):
    letters = Counter(line).most_common()
    letters = [letter for letter in letters if letter[0].isalpha()]
    letters = sorted(letters, key=cmp_to_key(sort_comp))
    return [letter for letter,_ in letters[:5]]

def valid_room(line):
    return "[{0}]".format(''.join(most_common(line[:-7], 5))) == line[-7:]

def rot_n(s, n):
    def lookup(v, n):
        o = ord(v)
        n = n % 26
        if v.isalpha():
            return chr((((o + n) - ord('a')) % 26) + ord('a'))
        if v == '-':
            return ' '
        return v
    return ''.join([lookup(char, n) for char in s])

def decrypt(room): # Rotate by sectorID
    return rot_n(room[:-11], int(room[-10:-7]))

with open('Day 4-Input') as file:
    lines = extract_file(file)
    lines = [line for line in lines if valid_room(line)]
    [print(line[-10:-7]) for line in lines if "north" in decrypt(line)]