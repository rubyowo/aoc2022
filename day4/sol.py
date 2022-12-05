def both(data):
    pairsp1 = 0
    pairsp2 = 0
    for line in data:
        r1, r2 = line.split(",")
        r1n1, r1n2 = r1.split("-")
        r2n1, r2n2 = r2.split("-")
    
        r1nums = list(num for num in range(int(r1n1), int(r1n2)+1))

        r2nums = list(num for num in range(int(r2n1), int(r2n2)+1))
        
        if((r1nums[0] in r2nums and r1nums[-1] in r2nums) or (r2nums[0] in r1nums and r2nums[-1] in r1nums)):
            pairsp1 += 1
        if(r1nums[0] in r2nums or r2nums[0] in r1nums):
            pairsp2 += 1

    print(pairsp1, pairsp2)

        
with open('input', 'r') as f:
    lines = f.readlines()
    both(lines)
