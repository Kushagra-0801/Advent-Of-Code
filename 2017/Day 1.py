input = 91212129

arr = list(map(int, list(str(input))))
res = 0
for i in range(0, len(arr)-1):
	# if its the last number then compare with the first one
	if(i+1 == len(arr)-1):
		if(arr[i+1] == arr[0]):
			res += arr[i+1]
	else:
		if(arr[i] == arr[i+1]):
			res += arr[i]
print(res)
