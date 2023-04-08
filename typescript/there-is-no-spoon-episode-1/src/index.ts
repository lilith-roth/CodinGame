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

interface gridPosition {
    x: number
    y: number
}

const grid: gridPosition[] = []

console.error(`w: ${width}, h: ${height}`)
for (let i = 0; i < height; i++) {
    const line: string = readline() // width characters, each either 0 or .
    console.error(line.toString())
    const position: string[] = line.split('')
}

// Write an action using console.log()
// To debug: console.error('Debug messages...');

// Three coordinates: a node, its right neighbor, its bottom neighbor
console.log('0 0 1 0 0 1')
