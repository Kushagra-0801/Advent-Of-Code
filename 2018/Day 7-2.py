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

current_time = 0
done_tasks = []
available_tasks = sorted(available_to_start)
workers = []
for i in range(0, 5):
	workers.append({'current_task': None, 'time_started': None, 'current_task_duration': None})

def analyze_done(worker, current_time, done_tasks):
	if worker['current_task'] is not None:
		if current_time - worker['time_started'] == worker['current_task_duration']:
			# task should pass in done
			task = worker['current_task']
			done_tasks.append(worker['current_task'])
			# reinit worker
			worker['current_task'] = None
			worker['time_started'] = None
			worker['current_task_duration'] = None
			return task
	return None

def pick_task(worker, available_tasks, index, current_time):
	if worker['current_task'] is None and len(available_tasks) > 0:
		# worker can pick task
		task = available_tasks[0]
		worker['current_task'] = task
		worker['time_started'] = current_time
		worker['current_task_duration'] = tasks.index(task) + 61
		available_tasks = available_tasks[1:]
		return task, available_tasks
	else:
		return None, None

while len(done_tasks) != len(tasks):
	# tasks are not ready yet
	# analyze current task and if it should pass in done
	taken_tasks = []
	done_tasks_this_second = []
	for i in range(0, 5):
		done = analyze_done(workers[i], current_time, done_tasks)
		if done:
			done_tasks_this_second.append(done)
	# recompute available tasks
	if done_tasks_this_second:
		for d in done_tasks:
			available_children = get_available_tasks(graph[d], done_tasks, graph)
			real_available_children = []
			for av in available_children:
				current_task = False
				for worker in workers:
					if worker['current_task'] == av:
						current_task = True
						break
				if not current_task:
					real_available_children.append(av)
			available_tasks = list(set(real_available_children)|set(available_tasks))
			available_tasks = sorted(available_tasks)
	for i in range(0, 5):
		task_taken, new_av = pick_task(workers[i], available_tasks, i, current_time)
		if task_taken:
			available_tasks = new_av
	available_tasks = sorted(available_tasks)
	if len(done_tasks) != len(tasks):
		current_time = current_time + 1
	
print(''.join(done_tasks))

print(current_time)
