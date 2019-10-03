unique_passphrases_count = 0
passphrases = []
with open('Day 4 - input', 'r') as f:
    for line in f:
        passphrase = list(line.split())
        if len(passphrase) == len(set(passphrase)):
            unique_passphrases_count += 1

print(unique_passphrases_count)
