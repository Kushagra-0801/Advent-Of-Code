with open('Day 1 - input', 'r') as f:
    k=f.readline()
    l=len(k)//2
    sum=0
    for i in range(l):
        if(k[i]==k[i+l]):
            sum = sum+ 2*int(k[i])
    print(sum)