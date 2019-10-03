from collections import defaultdict
import re

# read input
input_file = 'Day 7 - input'
with open(input_file) as f:
	content = f.readlines()

content = [l.strip() for l in content]

def get_parents(task, graph):
	parents = []
	for parent, children in graph.items():
		if task in children:
			parents.append(parent)
	return parents

def get_available_tasks(candidates, done_tasks, graph):
	# an available task has all his parents done and it isn't already visited
	available = []
	for task in candidates:
		parents = get_parents(task, graph)
		all_parents_done = all(parent in done_tasks for parent in parents)
		if all_parents_done and task not in done_tasks:
			available.append(task)
	return available

def visit(root_task, available_tasks, graph):
	if len(done_tasks) == len(tasks):
		return True
	else:
		done_tasks.append(root_task)
		available_children = get_available_tasks(graph[root_task], done_tasks, graph)
		available_tasks = list(set(available_children)|set(available_tasks))
		available_tasks = sorted(available_tasks)
		if available_tasks:
			root_task = available_tasks[0]
			available_tasks = available_tasks[1:]
			return visit(root_task, available_tasks, graph)
		else:
			return True

# Task id: list of tasks that depend on it
graph = defaultdict(list)

for line in content:
	result = re.search(r"Step ([A-Z]).*([A-Z])", line)
	parent = result.group(1)
	child = result.group(2)
	graph[parent].append(child)


# Find first task
# First task is the one that doesn't belong to any list of children
tasks = [chr(i) for i in range(ord('A'),ord('Z')+1)]

available_to_start = []

for t in tasks:
	found = False
	for children in graph.values():
		if t in children:
			found = True
			break
	if found:
		continue
	else:
		available_to_start.append(t)

available_to_start = sorted(available_to_start)

root_task = available_to_start[0]

available_tasks = available_to_start[1:]

available_tasks = sorted(available_tasks)

done_tasks = []

visit(root_task, available_tasks, graph)

print(''.join(done_tasks))
