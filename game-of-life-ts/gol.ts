type Offset = [number, number];
type Point = [number, number];
type Board = Point[];

const emptyBoard: Board = []
const offsetsAround: Offset[] = [[-1, -1], [ 0, -1], [ 1, -1],
                                 [-1,  0],           [ 1,  0],
                                 [-1,  1], [ 0,  1], [ 1,  1]]

const neighboursOf = (p: Point): Board =>
    offsetsAround.map(n => <Point>[p[0] + n[0], p[1] + n[1]]);

// arr.includes can't be used because [0, 0] !== [0, 0]
const contain = (arr: Board) => (pos: Point) =>
    arr.some(x => x[0] === pos[0] && x[1] === pos[1])

const rules = (count: number, isAlive: boolean) =>
    count === 3 || (count === 2 && isAlive)

const expandTestingArea = (generation: Board) =>
    generation.reduce((acc, current) =>
        acc.concat(neighboursOf(current).filter(x => !contain(acc)(x)))
        , emptyBoard)

const liveNeighboursInPrevGen = (generation: Board) => (current: Point) =>
    neighboursOf(current).filter(contain(generation)).length // filter use Currying

const gol = (generation: Board) => {
    const isInPrevGen = contain(generation)
    const noOfLiveNeigh = liveNeighboursInPrevGen(generation)
    const ruleForPosition = (p: Point) =>
        rules(noOfLiveNeigh(p), isInPrevGen(p))
    return expandTestingArea(generation).filter(ruleForPosition)
}

const view = (generation: Board) => {
    const width = 30;
    const height = 30;
    const rows: boolean[][] = Array(height)
    const avgX = Math.round(generation.reduce((acc, curr) => acc + curr[0], 0) / generation.length);
    const avgY = Math.round(generation.reduce((acc, curr) => acc + curr[1], 0) / generation.length);
    for (let point of generation) {
        const shiftX = point[0] - avgX + (width / 2);
        const shiftY = point[1] - avgY + (height / 2);
        if (shiftX > 0 && shiftY > 0 && shiftX < width && shiftY < height) {
            var row: boolean[] = rows[shiftY];
            if (!row) {
                row = Array(width)
                rows[shiftY] = row;
            }
            row[shiftX] = true;
            //console.log(`${shiftX} ${shiftY}`)
        }
    }

    var view = ""
    for (let row of rows) {
        var line = ""
        if (row) for (let cell of row) {
            line += cell
                ? "X"
                : " "
        }
        view += line + "\n";
    }
    return view;
}

const blinker = <Board>[[0, 0], [0, 1], [0, 2]];
const pentomimo = <Board>[[-1, 0], [-1, 1], [0, -1], [0, 0], [1, 0]];
var state = pentomimo;
var c = 0;
while (state.length > 0) {
    state = gol(state);

    process.stdout.write('\x1B[2J\x1B[0f');
    console.log("cells:"+state.length + " time:" + c)
    console.log(view(state))
    c++;

    Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 400);
}