input_file = 'Day 8 - input'
with open(input_file, 'r') as file:
    content = file.read()

data = [int(s) for s in content.split() if s.isdigit()]
metadata_sum = 0

class Node:
	def __init__(self, number_of_children, number_of_metadata, children, metadatas):
		self.number_of_children = number_of_children
		self.number_of_metadata = number_of_metadata
		self.children = children
		self.metadatas = metadatas

	def __str__(self):
		return str(self.number_of_children) + ' ' + str(self.number_of_metadata) + ' ' + str(self.children) + ' ' + str(self.metadatas)

	def __repr__(self):
		return str(self)

def process_part1():
	global data
	global metadata_sum
	child_count = data[0]
	meta_count = data[1]

	data = data[2:]

	if child_count == 0:
		for i in range(meta_count):
			metadata_sum += data[i]
		data = data[meta_count:]
	else:
		for i in range(child_count):
			process_part1()
		for i in range(meta_count):
			metadata_sum += data[i]
		data = data[meta_count:]

process_part1()

print(metadata_sum)
