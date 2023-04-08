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
    grid[y_index] = [];
    const line: string = readline() // width characters, each either 0 or .
    console.error(line.toString())
    const positions: string[] = line.split('');
    positions.forEach((value, x_index) => {
        let isPowered: boolean;
        if (value == "0") isPowered = true;
        else isPowered = false;
        // const newGridPosition:[number, number, boolean]
        //     = [x_index, y_index, isPowered];
        grid[y_index].push(isPowered);
        // grid.push(newGridPosition);
        console.error(`Storing ${x_index}, ${y_index}: ${grid[y_index][x_index]}}`);
    });
}

// Write an action using console.log()
// To debug: console.error('Debug messages...');

// Three coordinates: a node, its right neighbor, its bottom neighbor
console.log('0 0 1 0 0 1')
