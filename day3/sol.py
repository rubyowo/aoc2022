from string import ascii_letters
def letter_to_int(letter):
        return ascii_letters.index(letter) + 1

common = []
def part1(line):
    n = len(line)
    first = line[0:n//2]
    second = line[n//2:]

    norepeat = ""
    for c in first:
        if(c in second and c not in norepeat):
            norepeat += c
            common.append(c)

common2 = []
def part2(elfs):
        norepeat = ""
        for c in elfs[0].strip():
            if(c in elfs[1].strip() and c in elfs[2].strip() and c not in norepeat):
                norepeat += c
                common2.append(c)
    
with open('input', 'r') as f:
    lines = f.readlines()

    lines_iter = iter(lines)
    for elfs in zip(lines_iter, lines_iter, lines_iter):
        part2(elfs)

    for line in lines:
        part1(line)


    # Find priorities
    print(sum([letter_to_int(c) for c in common]))
    print(sum([letter_to_int(c) for c in common2]))
