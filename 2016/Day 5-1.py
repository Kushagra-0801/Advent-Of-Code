from hashlib import md5
import itertools

def extract_file(file):
    return file.readline()

with open('Day 5-Input') as file:
    line = extract_file(file)
    pw = []
    n = 8
    for i in itertools.count():
        cur = line + str(i)
        md5_hash = md5(cur.encode()).hexdigest()
        if md5_hash[:5] == "00000":
            pw.append(md5_hash[5:6])
        if len(pw) == n:
            break
    print(''.join(pw))