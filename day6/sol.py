def part1(data):
    for i in range(len(data) - 4):
        if len(set(data[i:i+4])) == 4:
            print(i+4)
            break

def part2(data):
    for i in range(len(data) - 14):
        if len(set(data[i:i+14])) == 14:
            print(i+14)
            break

with open('./input', 'r') as f:
#    data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
    data = f.read().strip()
    part1(data)
    part2(data)

