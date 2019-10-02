from hashlib import md5
import itertools

def extractFile(file):
    return file.readline()

def prettyPrint(pw, n):
    s = "________"
    for i in range(n):
        if pw[i]:
            s = s[:i] + pw[i] + s[i + 1:] # replace string at i with char
    print(s)

with open('Day 5-Input') as file:
    line = extractFile(file)
    n = 8
    pw = [None] * n
    for i in itertools.count():
        cur = line + str(i)
        md5Hash = md5(cur.encode()).hexdigest()
        if md5Hash[:5] == "00000":
            index = md5Hash[5:6]
            if index >= '0' and index < str(n):
                index = int(index)
                if not pw[index]: # not already written
                    pw[index] = md5Hash[6:7]
                    prettyPrint(pw, n)
        if not None in pw:
            break
    print(''.join(pw))
