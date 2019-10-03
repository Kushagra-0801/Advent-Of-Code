valid_passphrases_count = 0
passphrases = []

with open('Day 4 - input', 'r') as f:
    for line in f:
        passphrase_list = list(line.split())
        passphrase_list = [('').join(sorted(passphrase)) for passphrase in passphrase_list]
        if len(passphrase_list) == len(set(passphrase_list)):
            valid_passphrases_count += 1

print(valid_passphrases_count)
