def is_nice(string):
    num_vowels = 0
    twice_in_row = False
    vowels = ('a', 'e', 'i', 'o', 'u')
    prohibited_strings = ('ab', 'cd', 'pq', 'xy')
    s1, s2 = iter(string), iter(string)
    if next(s2) in vowels:
        num_vowels = 1
    for char1, char2 in zip(s1, s2):
        if char1+char2 in prohibited_strings:
            return False
        if char2 in vowels:
            num_vowels += 1
        if char1 == char2:
            twice_in_row = True
    if twice_in_row and num_vowels >= 3:
        return True

with open('Day 5 - input', 'r') as f:
    nice_strings = 0
    for string in f:
        if is_nice(string):
            nice_strings += 1
print(nice_strings)
