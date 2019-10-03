import itertools

valid_passphrases_count = 0
passphrases = []

with open('Day 4 - input', 'r') as f:
    for line in f:
        passphrases.append(list(line.split()))
    for passphrase_list in passphrases:
        passphrase = list(map(lambda x: ('').join(sorted(list(x))), passphrase_list))
        if len(passphrase) == len(set(passphrase)):
            valid_passphrases_count += 1
        

print(valid_passphrases_count)
