import re

ans = 0
skip =False
with open("input.txt", "r") as f:
    total = f.read()
    x= re.findall("(mul\(\d{1,3},\d{1,3}\)|don\'t|do)", total)
    for entry in x:
        if entry == "don't":
            skip = True
        elif entry == "do":
            skip = False
            continue
        if skip:
            continue
        nums = re.findall("\d{1,3}", entry)
        product = int(nums[0])*int(nums[1])
        # print(product)
        ans+=product

print(ans)