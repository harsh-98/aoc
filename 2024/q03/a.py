import re

ans = 0
with open("input.txt", "r") as f:
    total = f.read()
    x = re.findall("mul\(\d{1,3},\d{1,3}\)", total)
    for entry in x:
        nums = re.findall("\d{1,3}", entry)
        product = int(nums[0])*int(nums[1])
        # print(product)
        ans+=product

print(ans)