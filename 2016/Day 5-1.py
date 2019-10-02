from hashlib import md5
import itertools

def extractFile(file):
    return file.readline()

with open('Day 5-Input') as file:
    line = extractFile(file)
    pw = []
    n = 8
    for i in itertools.count():
        cur = line + str(i)
        md5Hash = md5(cur.encode()).hexdigest()
        if md5Hash[:5] == "00000":
            pw.append(md5Hash[5:6])
        if len(pw) == n:
            break
    print(''.join(pw))
