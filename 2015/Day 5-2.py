def is_nice(string):
    two_letters_repeat = False
    one_letter_repeat = False
    for i in range(len(string)-2):
        if string[i] == string[i+2]:
            one_letter_repeat = True
        if string[i:i+2] in string[i+2:]:
            two_letters_repeat = True
    if one_letter_repeat and two_letters_repeat:
        return True
    return False


with open('Day 5 - input', 'r') as f:
    nice_strings = 0
    for string in f:
        if is_nice(string):
            nice_strings += 1
print(nice_strings)
