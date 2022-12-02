with open('./input', 'r') as f:
    inputf = f.read()
    elves = inputf.split("\n\n")

    total_calories = [s.strip().split('\n') for s in elves]
    calories_per_elf = [0] * len(elves)

    for i in range(0, len(elves)):
        calories_per_elf[i] += sum([int(c) for c in total_calories[i]])

    # Part 1
    print(max(calories_per_elf))

    # Part 2
    print(sum(sorted(calories_per_elf, reverse=True)[:3]))
