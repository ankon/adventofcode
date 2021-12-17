#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

type Dot = [row: number, col: number];
interface Fold {
	/** Coordinate at which to fold */
	coord: number;
	/** Axis index into the dot structure: '0' means mirror the row, '1' means mirror the column */
	index: number;
}

function printDots(
	caption: string,
	dots: Dot[],
	{
		maxRow,
		maxCol,
		foldRow = -1,
		foldCol = -1,
	}: { maxRow: number; maxCol: number; foldRow?: number; foldCol?: number }
) {
	console.log(caption);
	// Really inefficient logic :)
	for (let row = 0; row < maxRow; row++) {
		if (row === foldRow) {
			console.log(''.padEnd(maxCol + 1, '-'));
			continue;
		}

		const cols = dots
			.filter(([dotRow]) => row === dotRow)
			.map(([, dotCol]) => dotCol)
			.sort((a, b) => b - a);
		let line = '';
		let col;
		while (typeof (col = cols.pop()) !== 'undefined') {
			line += '#'.padStart(col - line.length + 1, '.');
		}

		let paddedLine = line.padEnd(maxCol + 1, '.');
		if (foldCol !== -1) {
			paddedLine =
				paddedLine.slice(0, foldCol) +
				'|' +
				paddedLine.slice(foldCol + 1);
		}
		console.log(paddedLine);
	}
}

function applyFold(
	dots: Dot[],
	{ index: axis, coord }: Fold,
	{ maxRow, maxCol }: { maxRow: number; maxCol: number }
): Dot[] {
	// Folding is done by "mirroring" the coordinates along the axis, and removing
	// any duplicate coordinate pairs. The duplicate detection is done by
	// calculating a single coordinate-index from the row/col: index = row * (maxCol + 1) + col,
	// and keeping those in a set.
	const printOpts = {
		maxRow,
		maxCol,
		foldCol: axis === 1 ? coord : -1,
		foldRow: axis === 0 ? coord : -1,
	};
	printDots(`before fold ${axis ? 'x' : 'y'} at ${coord}`, dots, printOpts);
	const result: Dot[] = [];
	const seenPairs = new Set();
	for (const dot of dots) {
		let newDot: Dot;
		if (dot[axis] < coord) {
			// Retain
			newDot = dot;
		} else {
			// Mirror on axis
			newDot = [0, 0];

			newDot[axis] = coord - (dot[axis] - coord);
			const otherAxis = (axis + 1) % 2;
			newDot[otherAxis] = dot[otherAxis];
		}

		const index = newDot[0] * (maxCol + 1) + newDot[1];
		if (!seenPairs.has(index)) {
			seenPairs.add(index);
			result.push(newDot);
		}
	}
	printDots('after', result, printOpts);
	return result;
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	const dots: Dot[] = [];
	const folds: Fold[] = [];
	let maxRow = 0;
	let maxCol = 0;

	return new Promise((resolve, reject) => {
		rl.on('line', (line) => {
			if (!line) {
				return;
			}

			const [colS, rowS] = line.split(/,/);
			if (rowS) {
				const col = parseInt(colS, 10);
				const row = parseInt(rowS, 10);
				maxRow = Math.max(maxRow, row);
				maxCol = Math.max(maxCol, col);
				dots.push([row, col]);
			} else {
				const [instructionAndAxis, coord] = line.split(/=/);
				folds.push({
					coord: parseInt(coord, 10),
					index: instructionAndAxis === 'fold along x' ? 1 : 0,
				});
			}
		});
		rl.on('error', (err) => {
			reject(err);
		});
		rl.on('close', () => {
			const afterFirstFold = applyFold(dots, folds[0], {
				maxRow,
				maxCol,
			});
			console.log(`Results for ${input}: ${afterFirstFold.length} dots`);
			if (input === '-example') {
				// Debug side-effect: Print the complete folding result.
				applyFold(afterFirstFold, folds[1], { maxRow, maxCol });
			}
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

const INPUT_SPECS = ['-example', ''];

main(
	INPUT_SPECS.map(
		(inputSpec) =>
			`${basename(
				process.argv[1],
				extname(process.argv[1])
			)}${inputSpec}.input`
	)
).catch((err) => console.error(err));
