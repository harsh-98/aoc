a = [] 
b = {}
with open('input.txt', 'r') as f:
    line = f.readline()
    while line != "":
        x = line.split("   ")
        a.append(int(x[0]))
        e= int(x[1][:-1])
        b[e] = b.get(e, 0)+1
        line =  f.readline()

a.sort()
# b.sort()

s=0
for ind, e in enumerate(a): 
    s+=b.get(e,0)*e
    # s+=abs(e- b[ind])
print(s)