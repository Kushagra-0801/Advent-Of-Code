input_file = 'Day 5 - input'
with open(input_file, 'r') as file:
    content = file.read()

def polymerize(content):
	size = len(content)
	i = 0
	while i < size - 1:
		if content[i].lower() == content[i+1].lower() and content[i] != content[i+1]:
			if i + 2 <= size - 1:
				content = content[:i] + content[i+2:]
			else:
				content = content[:i]
			# looking at neighbours
			if i >= 1:
				i -= 1
			size = len(content)
		else: 
			i += 1
	return size

print(polymerize(content))