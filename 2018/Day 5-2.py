import re

input_file = 'Day 5 - input'
with open(input_file, 'r') as file:
    content = file.read()

alphabet = list(map(chr, range(97, 123)))
min_size = len(content) + 1
original_content = content

for c in alphabet:
	local_content = original_content
	regex = r"[" + re.escape(c) + re.escape(c.upper()) + "]"
	local_content = re.sub(regex, '', local_content)
	size = polymerize(local_content)
	if size < min_size:
		min_size = size

print(min_size)