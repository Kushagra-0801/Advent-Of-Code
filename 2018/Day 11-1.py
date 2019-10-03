from collections import defaultdict
import operator

# for this day input didn't come as a file to read
grid_serial_number = 6548

def get_hundreds_digit(number):
	if number < 100:
		return 0
	else:
		number_s = str(number)
		hundreds_digit = number_s[-3]
		return int(hundreds_digit)

def compute_power_level(x, y):
	rack_id = (x + 1) + 10
	power_level = (rack_id * (y + 1)) + grid_serial_number
	power_level = power_level * rack_id
	power_level = get_hundreds_digit(power_level)
	power_level -= 5
	return power_level

def compute_square_sum(x, y, size, matrix):
	x_start = x
	x_end = x_start + size - 1
	y_start = y
	y_end = y_start + size - 1
	square_sum = 0
	for i in range(x_start, x_end + 1):
		for j in range(y_start, y_end+1):
			square_sum += matrix[i][j]
	return square_sum

# create a 300 x 300 matrix
matrix = []
for x in range(0, 300):
	l = []
	for y in range(0, 300):
		power_level = compute_power_level(x, y)
		l.append(power_level)
	matrix.append(l)

# find the left top coordinate witch has the maximum power
power_levels = defaultdict(int)

for x in range(0, 300 - 2):
	for y in range(0, 300 - 2):
		square_sum = compute_square_sum(x, y, 3, matrix)
		power_levels[(x, y)] = square_sum

coordinate = max(power_levels.items(), key=operator.itemgetter(1))[0]
print(coordinate[0] + 1, coordinate[1] + 1)