def increment(password: list) -> list:
    password = password[:]
    i = len(password) - 1
    while password[i] == 'z' and i != -1:
        i -= 1
    if i == -1:
        return ['a' for _ in range(len(password)+1)]
    else:
        password[i] = chr(ord(password[i])+1)
        for j in range(i+1, len(password)):
            password[j] = 'a'
    return password


def get_three(password):
    a1, a2, a3 = iter(password), iter(password), iter(password)
    next(a3)
    next(a3)
    next(a2)
    return zip(a1, a2, a3)


def two_doubles(password):
    i = 0
    got_double = False
    while i < len(password)-1:
        if password[i] == password[i+1]:
            if got_double:
                return True
            else:
                got_double = True
                i += 1
        i += 1


NEW_PASSWORD_ITERATION = 2
cycled_passwords = 1

with open('Day 11 - input', 'r') as f:
    password = list(f.readline().strip())
    while True:
        password = increment(password)
        if 'i' in password or 'o' in password or 'l' in password:
            continue
        if not two_doubles(password):
            continue
        for a, b, c in get_three(password):
            if ord(a)+2 == ord(b)+1 == ord(c):
                if cycled_passwords < NEW_PASSWORD_ITERATION:
                    cycled_passwords += 1
                else:
                    print(''.join(password))
                    quit()
