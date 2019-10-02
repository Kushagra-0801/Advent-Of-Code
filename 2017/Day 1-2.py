f = open("input.txt", 'r')
input = f.read()

arr = list(map(int, list(str(input))))
range_match = int(len(arr) / 2)
res = 0

for i in range(0, len(arr)):
	if(i + range_match > len(arr)):
		remain_amount = (range_match + i) - len(arr)
		if(arr[i] == arr[remain_amount]):
			res += arr[i]
	elif(i+range_match == len(arr)):
		if(arr[i] == arr[0]):
			res += arr[i]
	else:
		if(arr[i] == arr[i+range_match]):
			res += arr[i]

print(res)
