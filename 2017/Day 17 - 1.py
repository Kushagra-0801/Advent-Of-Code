import collections


with open('Day 17 - input', 'r') as f:
    data = f.readlines()

steps = int(data[0])
circular_buffer = collections.deque([0])
insertion_count = 2017

for i in range(1, insertion_count + 1):
    # rotate buffer steps modulo current length
    circular_buffer.rotate(-steps % i)
    # append new value
    circular_buffer.append(i)

# solution is first value in buffer
print(f"The value after {insertion_count} is {circular_buffer[0]}")
