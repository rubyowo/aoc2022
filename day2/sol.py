opp = {
    'A': 1,
    'B': 2,
    'C': 3
}

you = {
     'X': 1,
     'Y': 2,
     'Z': 3
}

winning = {
    'Y': 1,
    'Z': 2,
    'X': 3,
} # Paper(Y) wins against Rock(A), Scissors(Z) wins against Paper(B), Rock(X) wins against Scissors(C).

outcomes = {
    'X': 0,
    'Y': 3,
    'Z': 6
}

# Scuffed, I know but w/e
results = {
    "A X":0+3,
    "A Y":3+1,
    "A Z":6+2,
    "B X":0+1,
    "B Y":3+2,
    "B Z":6+3,
    "C X":0+2,
    "C Y":3+3,
    "C Z":6+1,
}

def part1(data):
    points = 0
    for line in data:
        f, s = line.split()
        if opp.get(f) == you.get(s):
            points += outcomes['Y']
        elif opp.get(f) == winning.get(s):
            points += outcomes['Z']
        # Don't do anything if you lose since its just desicion+0.
        points += you.get(s)
    print(points)

def part2(data):
    points = 0
    for line in data:
        points += results[line]
    print(points)

with open('input', 'r') as f:
    data = f.read().strip().splitlines()
    part1(data)
    part2(data)
