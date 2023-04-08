// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck

/**
 * Don't let the machines win. You are humanity's last hope...
 *
 * Info reg. ts-nocheck:
 *  When coding using a local IDE instead of the built-in one on CodinGame,
 *  you'll get quite a lot of TypeScript errors, due to functions existing implicitly.
 **/

const width: number = parseInt(readline()) // the number of cells on the X axis
const height: number = parseInt(readline()) // the number of cells on the Y axis

// Each tuple in the grid contains x coordinate, y coordinate & if it is powered.
const grid: boolean[][] = []

console.error(`w: ${width}, h: ${height}`)
for (let y_index = 0; y_index < height; y_index++) {
    grid[y_index] = []
    const line: string = readline() // width characters, each either 0 or .
    console.error(line.toString())
    const x_positions: string[] = line.split('')
    x_positions.forEach((value, x_index) => {
        let isPowered: boolean
        if (value == '0') isPowered = true
        else isPowered = false
        grid[y_index].push(isPowered)
        console.error(
            `Storing ${x_index}, ${y_index}: ${grid[y_index][x_index]}}`
        )
    })
}

let outputs: string[] = [];

for (let y = 0; y < grid.length; y++) {
    for (let x = 0; x < grid[y].length; x++) {
        // Ignore if the current node isn't active itself.
        if (!grid[y][x]) continue
        console.error(`Node found at ${x} ${y}`);
        let currentOutput: string = `${x} ${y} `;

        // Searching for next active neighbor on X axis
        let foundX: boolean = false;
        for (let i = x+1; i < width; i++) {
            if (grid[y][i]) {
                currentOutput += `${i} ${y} `;
                foundX = true;
            }
            if (foundX) break;
        }
        if (!foundX) currentOutput += "-1 -1 ";

        // Searching for next active neighbor on Y axis
        let foundY: boolean = false;
        for (let i = y+1; i < height; i++) {
            if (grid[i][x]) {
                currentOutput += `${x} ${i}`;
                foundY = true;
            }
            if (foundY) break;
        }
        if (!foundY) currentOutput += "-1 -1";
        console.error(`Adding output: ${currentOutput}`)
        outputs.push(currentOutput);
    }
}

// Write an action using console.log()
// To debug: console.error('Debug messages...');

// Three coordinates: a node, its right neighbor, its bottom neighbor
//console.log('0 0 1 0 0 1')
outputs.forEach((output, i) => {
    console.log(output);
});
