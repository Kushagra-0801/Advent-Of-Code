unique_passphrases_count = 0
passphrases = []
with open('Day 4 - input', 'r') as f:
    for line in f:
        passphrases.append(list(line.split()))
    for passphrase in passphrases:
        if len(passphrase) == len(set(passphrase)):
            unique_passphrases_count += 1
print(unique_passphrases_count)