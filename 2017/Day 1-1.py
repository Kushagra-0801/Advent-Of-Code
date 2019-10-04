with open('Day 1 - input', 'r') as f:
    k=f.readline()
    sum=0
    i=0
    for i in range(0,len(k)-1):
        if(k[i]==k[i+1]):
            sum = sum + int(k[i])
    print(i)
    if(i==len(k)-2):
        if(k[i+1]==k[0]):
            sum = sum + int(k[i+1])
            print('lo')
    print(sum)