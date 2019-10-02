import string
import itertools
#a Pyython2 solution
lines = []
with open('Day 2 - input') as fp:
	for line in fp:
		lines.append(map(int, string.split(line.strip(), '\t')))

part1 = 0
part2 = 0
for line in lines:
	part1 += max(line) - min(line)

	for i in itertools.combinations(line, 2):
		if max(i) % min(i) == 0:
			part2 += max(i) / min(i)
			break

print ('Part 1: %s' % part1)