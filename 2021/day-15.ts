#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

const DEBUG = process.env.DEBUG ?? false;

type Pos = [row: number, col: number];

/** 2D structure with costs to enter a particular tile */
type Maze = number[][];

/** Node inside the open/closed lists */
interface Node {
	pos: Pos;
	cost: number;
	estimatedCost: number;
	path: Pos[];
}

function printMaze(maze: Maze, open: Node[], closed: Node[], path?: Pos[]) {
	if (!DEBUG) {
		return;
	}
	// Draw the maze, possibly highlighting the path
	function label(pos: Pos, cost: number): string {
		if (path) {
			const pathIndex = path.findIndex(
				([row, col]) => pos[0] === row && pos[1] === col
			);
			if (pathIndex > -1) {
				return pathIndex === path.length - 1
					? `>${cost}<`
					: `.${cost}.`;
			}
		}
		const isOpen =
			open.findIndex(
				({ pos: [row, col] }) => pos[0] === row && pos[1] === col
			) > -1;
		if (isOpen) {
			return `(${cost})`;
		}
		const isClosed =
			closed.findIndex(
				({ pos: [row, col] }) => pos[0] === row && pos[1] === col
			) > -1;
		if (isClosed) {
			return ` ${cost} `;
		}
		return ` ${cost} `;
	}

	for (let row = 0; row < maze.length; row++) {
		let line = '';
		for (let col = 0; col < maze[row].length; col++) {
			line += label([row, col], maze[row][col]);
		}
		console.log(line);
	}
}

function costPerStepEstimatePathAverage(path: Pos[], maze: Maze) {
	if (path.length === 0) {
		return 1;
	} else {
		const costOfPath = path.reduce(
			(c, [row, col]) => c + maze[row][col],
			0
		);
		return costOfPath / path.length;
	}
}

function costPerStepEstimateConstant() {
	return 1;
}

/** Estimate the costs between `from` and `to` */
function estimateCost(
	maze: Maze,
	from: Pos,
	to: Pos,
	pathToFrom: Pos[],
	costPerStepEstimate: (path: Pos[], maze: Maze) => number
): number {
	const requiredSteps =
		Math.abs(to[0] - from[0] + 1) + Math.abs(to[1] - from[1] + 1);
	return requiredSteps * costPerStepEstimate(pathToFrom, maze);
}

/**
 * Find the cheapest path from `from` to `to` and return the cost of that
 *
 * @param maze
 * @param from
 * @param to
 */
function findPath(
	maze: Maze,
	from: Pos,
	to: Pos,
	costPerStepEstimate: (path: Pos[], maze: Maze) => number
): number {
	const open: Node[] = [
		{
			pos: from,
			cost: 0,
			estimatedCost: estimateCost(
				maze,
				from,
				to,
				[],
				costPerStepEstimate
			),
			path: [from],
		},
	];
	const closed: Node[] = [];

	let steps = 0;
	while (open.length > 0) {
		// Pick (and remove) the open node with the lowest total (up-to + estimated) cost
		const cheapestIndex = open.reduce(
			(r, _n, i) =>
				open[r].cost + open[r].estimatedCost <=
				open[i].cost + open[i].estimatedCost
					? r
					: i,
			0
		);
		const cheapest = open.splice(cheapestIndex, 1)[0];

		printMaze(maze, open, closed, cheapest.path);
		console.debug(
			`After ${steps} steps: ${cheapest.path.length} length, ${cheapest.cost} so far, ${cheapest.estimatedCost} still to go`
		);
		steps++;

		// Generate the successors (non-diagonal)
		for (const delta of [
			[-1, 0],
			[0, 1],
			[1, 0],
			[0, -1],
		]) {
			const successorPos: Pos = [
				cheapest.pos[0] + delta[0],
				cheapest.pos[1] + delta[1],
			];
			if (
				successorPos[0] < 0 ||
				successorPos[0] >= maze.length ||
				successorPos[1] < 0 ||
				successorPos[1] >= maze.length
			) {
				// Not on the map
				continue;
			}

			const successorCost =
				cheapest.cost + maze[successorPos[0]][successorPos[1]];
			// 1. hit the end? then this must be the cheapest path
			if (successorPos[0] === to[0] && successorPos[1] === to[1]) {
				console.log(`Path at ${successorCost}`, cheapest.path);
				return successorCost;
			}

			// 2. pos is already in the open list? if that is cheaper ignore this, otherwise replace
			// 3. pos is already in the closed list? if that is cheaper ignore this, otherwise add to open
			const successorPath = [...cheapest.path, successorPos];
			const successorEstimatedCost = estimateCost(
				maze,
				successorPos,
				to,
				successorPath,
				costPerStepEstimate
			);
			const successor = {
				pos: successorPos,
				cost: successorCost,
				estimatedCost: successorEstimatedCost,
				path: successorPath,
			};
			const openSuccessorIndex = open.findIndex(
				({ pos: p }) =>
					p[0] === successorPos[0] && p[1] === successorPos[1]
			);
			if (openSuccessorIndex > -1) {
				// This position exists, check whether we found a cheaper or at least equally
				// expensive shorter path to it.
				if (
					open[openSuccessorIndex].cost +
						open[openSuccessorIndex].estimatedCost >=
					successorCost + successorEstimatedCost
				) {
					if (
						open[openSuccessorIndex].path.length >
						successor.path.length
					) {
						open[openSuccessorIndex] = successor;
					}
				}
				continue;
			}
			const closedSuccessorIndex = closed.findIndex(
				({ pos: p }) =>
					p[0] === successorPos[0] && p[1] === successorPos[1]
			);
			if (
				closedSuccessorIndex > -1 &&
				closed[closedSuccessorIndex].cost +
					closed[closedSuccessorIndex].estimatedCost <=
					successorCost + successorEstimatedCost
			) {
				// No point to look at this, we have been here before with a lower cost already.
				continue;
			}

			// Add to the open list to check it.
			open.push(successor);
		}

		// Remember that we have been here.
		closed.push(cheapest);
	}

	throw new Error('No open nodes left, but end not found?');
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	return new Promise((resolve, reject) => {
		const maze: Maze = [];

		rl.on('line', (line) => {
			maze.push(Array.from(line.trim()).map((c) => parseInt(c, 10)));
		});
		rl.on('error', (err) => {
			reject(err);
		});
		rl.on('close', () => {
			let costPerStepEstimate;
			switch (process.env.ESTIMATOR) {
				case 'avg':
					costPerStepEstimate = costPerStepEstimatePathAverage;
					break;
				case 'constant':
				default:
					costPerStepEstimate = costPerStepEstimateConstant;
					break;
			}
			// NB: Maze is square by definition
			const riskLevel = findPath(
				maze,
				[0, 0],
				[maze.length - 1, maze.length - 1],
				costPerStepEstimate
			);
			console.log(`Results for ${input}: ${riskLevel}`);

			resolve();
		});
	});
}

async function main(inputFiles: string[]) {
	for (const inputFile of inputFiles) {
		try {
			await processInput(inputFile);
		} catch (err: any) {
			console.error(`Cannot process ${inputFile}: ${err.message}`);
		}
	}
}

const INPUT_SPECS = [
	//
	//'-example',
	'',
];

main(
	INPUT_SPECS.map(
		(inputSpec) =>
			`${basename(
				process.argv[1],
				extname(process.argv[1])
			)}${inputSpec}.input`
	)
).catch((err) => console.error(err));
