const input = await Deno.readTextFile('./input')

const [table, instrlines] = input.split('\n\n')

// Lebster's input parsing
const rows = table.split('\n').slice(0, -1).map(line => line
    .replaceAll("    ", "-") // Four spaces indicates blank
	.replaceAll(" ", "") // Other spaces are separators
	.replaceAll("[", "") // Brackets are decorative
	.replaceAll("]", "")
	.split("")); // Split the string into an array of characters

const part1_towers = Array.from({ length: rows[0].length }, () => []);
const part2_towers = Array.from({ length: rows[0].length }, () => []);
for (let i = 0; i < rows[0].length; i++) {
	for (const row of rows) {
		if (row[i] !== '-') {
			part1_towers[i].push(row[i]);
            part2_towers[i].push(row[i]);
		}
	}
}

for(const line of instrlines.split('\n')) {
    let [n, src, dest]  = line.match(/\d+/g).filter(m => m !== null);
    n = Number(n)
    src = Number(src) -1
    dest = Number(dest) - 1

    const p1 = part1_towers[src].splice(0, n).reverse();
	part1_towers[dest].unshift(...p1);

    const p2 = part2_towers[src].splice(0, n);
	part2_towers[dest].unshift(...p2);
}

console.log(part1_towers.map(t => t[0]).join(''));
console.log(part2_towers.map(t => t[0]).join(''));
