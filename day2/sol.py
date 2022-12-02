opp = {
    'A': 1,
    'B': 2,
    'C': 3,
}

you = {
    'X': 1,
    'Y': 2,
    'Z': 3,
}

# Part 1 logic
def rps_logic(op, yu):
    if(op == 1 and yu == 2):
        return 2+6
    elif(op == 1 and yu == 3):
        return 3+0
    elif(op == 1 and yu == 1):
        return 1+3

    elif(op == 2 and yu == 3):
        return 3+6
    elif(op == 2 and yu == 1):
        return 1+0
    elif(op == 2 and yu == 2):
        return 2+3

    elif(op == 3 and yu == 1):
        return 1+6
    elif(op == 3 and yu == 2):
        return 2+0
    elif(op == 3 and yu == 3):
        return 6

# Part 2 logic
round_outcomes = {
        'X': 0,
        'Y': 3,
        'Z': 6
}

def rps_logic2(op, ou):
    if(op == 1 and ou == 0):
        return 3+0
    elif(op == 1 and ou == 3):
        return 1+3
    elif(op == 1 and ou == 6):
        return 2+6

    elif(op == 2 and ou == 0):
        return 1+0
    elif(op == 2 and ou == 3):
        return 2+3
    elif(op == 2 and ou == 6):
        return 3+6

    elif(op == 3 and ou == 0):
        return 2+0
    elif(op == 3 and ou == 3):
        return 3+3
    elif(op == 3 and ou == 6):
        return 1+6

with open('input', 'r') as f:
    final_score = 0
    final_score2 = 0

    for line in f.read().strip().split("\n"):

        first_col = opp.get(line[0])

        # Part 1
        second_col = you.get(line[2])
        final_score += rps_logic(first_col, second_col)

        # Part 2
        second_col2 = round_outcomes.get(line[2])
        final_score2 += rps_logic2(first_col, second_col2)

    print(final_score)
    print(final_score2)
