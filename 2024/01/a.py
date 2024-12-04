a = [] 
b = []
with open('input.txt', 'r') as f:
    line = f.readline()
    while line != "":
        x = line.split("   ")
        a.append(int(x[0]))
        b.append(int(x[1][:-1]))
        line =  f.readline()

a.sort()
b.sort()

s=0
for ind, e in enumerate(a): 
    s+=abs(e- b[ind])
print(s)