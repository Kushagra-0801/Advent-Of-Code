input_file = 'Day 8 - input'
with open(input_file, 'r') as file:
    content = file.read()

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

data2 = [int(s) for s in content.split() if s.isdigit()]

nodes = []

# Builds the tree
def process_part2(parent):
	global data2
	global nodes

	child_count = data2[0]
	meta_count = data2[1]

	if len(nodes) == 0:
		node = Node(child_count, meta_count, [], [])
		nodes.append(node)

	else:
		node = Node(child_count, meta_count, [], [])
		parent.children.append(node)

	data2 = data2[2:]

	if child_count == 0:
		for i in range(meta_count):
			node.metadatas.append(data2[i])
		data2 = data2[meta_count:]
	else:
		for i in range(child_count):
			process_part2(node)
		for i in range(meta_count):
			node.metadatas.append(data2[i])
		data2 = data2[meta_count:]

process_part2(None)

root_value = 0

def compute_value(node):
	global root_value

	if node.number_of_children == 0:
		root_value += sum(node.metadatas)

	else:
		for i in range(node.number_of_metadata):
			if node.metadatas[i] != 0 and node.metadatas[i] - 1 <= node.number_of_children - 1:
				compute_value(node.children[node.metadatas[i] - 1])

compute_value(nodes[0])

print(root_value)
