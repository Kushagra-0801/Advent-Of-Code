from itertools import count
from hashlib import md5

with open('Day 4 - input', 'r') as f:
    message = f.readline().strip()
    for num in count(1):
        m = md5(f'{message}{num}'.encode('utf-8'))
        if m.hexdigest().startswith('00000'):
            print(num)
            break
