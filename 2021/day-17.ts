#!/usr/bin/env node

interface Area {
	xMin: number;
	xMax: number;
	yMin: number;
	yMax: number;
}

type Pos = [number, number];

/** Show the graph with the given area and initial vector */
function showGraph(
	{ xMin, xMax, yMin, yMax }: Area,
	[dx, dy]: Pos,
	[xInitial, yInitial] = [0, 0]
): number {
	// Simulate until the position definitely doesn't hit the target area: y < yMax (we're in the depth!), or x > xMax
	// XXX: Negative area?
	let x = xInitial;
	let y = yInitial;
	const steps: Pos[] = [];
	let missed = false;
	while (true) {
		steps.push([x, y]);
		console.log([x, y]);

		// Check if we hit the target already
		if (x > xMin && x < xMax && y > yMin && y < yMax) {
			console.log(`Target hit!`);
			break;
		}

		// Calculate next point and check whether we can stop
		y = y + dy;
		x = x + dx;
		if (x > xMax || y < Math.min(yMin, yMax)) {
			console.log(`Target missed`);
			missed = true;
			break;
		}

		// Update speed
		if (dx > 0) {
			dx -= 1;
		} else if (dx < 0) {
			dx += 1;
		}
		dy--;
	}

	const maxHeight = steps.reduce((h, [, y]) => (y > h ? y : h), 0);

	// TODO: Display the graph for the human to adjust the aim
	return missed ? -maxHeight - 1 : maxHeight;
}

function processInput(input: string): void {
	const INPUT_RE =
		/target area: x=(?<xMin>-?\d+)\.\.(?<xMax>-?\d+), y=(?<yMin>-?\d+)..(?<yMax>-?\d+)/g;
	const match = INPUT_RE.exec(input);
	if (!match) {
		throw new Error(`Cannot parse input`);
	}
	const area = Object.fromEntries(
		Object.entries(match.groups!).map(([k, v]) => [k, parseInt(v, 10)])
	) as unknown as Area;
	const initial: Pos = [10, 10];
	const height = showGraph(area, initial);
	console.log(`Reached height of ${height} with ${initial} for ${input}`);
}

const INPUTS = [
	//
	'target area: x=20..30, y=-10..-5',
	'target area: x=253..280, y=-73..-46',
];

INPUTS.forEach(processInput);
