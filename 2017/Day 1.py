number = input()
number = str(number)
sum = 0

l = len(number)

for i in range(l):
  if(number[i] == number[(i+1)%l]):
    sum += 1
    
    
print(sum)
