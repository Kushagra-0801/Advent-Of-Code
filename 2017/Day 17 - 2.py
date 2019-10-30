import collections


with open('Day 17 - input', 'r') as f:
    data = f.readlines()

steps = int(data[0])

circular_buffer = collections.deque([0])
insertion_count = 5000000
offset = 0

for i in range(1, insertion_count + 1):
    # rotate buffer steps modulo current length
    spin = -steps % i
    circular_buffer.rotate(spin)
    # append new value
    circular_buffer.append(i)
    # keep track of cummulative rotation
    offset = (offset + spin) % i

# roll back to get value after 0
circular_buffer.rotate(-offset - 1)
print(f"The value after 0 the moment {insertion_count} is inserted: {circular_buffer[0]}")
