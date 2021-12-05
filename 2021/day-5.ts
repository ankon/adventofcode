#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

type Point = { x: number; y: number };
type LineSegment = { start: Point; end: Point };
/**
 * Index is y*MAX_X+x, mapped value is number of line segments at that point
 */
type Floor = Map<number, number>;
interface FloorOptions {
	maxX: number;
	maxY: number;
}

function parsePoint(s: string): Point {
	const [x, y] = s
		.trim()
		.split(/,/)
		.map((v) => parseInt(v, 10));
	return { x, y };
}

function parseLineSegment(line: string): LineSegment {
	const [p1, p2] = line.split(/\s*->\s*/);
	return {
		start: parsePoint(p1),
		end: parsePoint(p2),
	};
}

function toString({ x, y }: Point): string {
	return `${x},${y}`;
}

function printFloor(floor: Floor, { maxX, maxY }: FloorOptions): void {
	for (let y = 0; y < maxY; y++) {
		let line = '';
		for (let x = 0; x < maxX; x++) {
			const v = floor.get(y * maxX + x);
			line += String(v ?? '.');
		}
		console.log(line);
	}
}

function updateFloor(
	floor: Floor,
	{ start, end }: LineSegment,
	{ maxX, maxY }: FloorOptions
): boolean {
	// Find the index delta between the two points, and then update the floor
	// at all points touched by the line.
	// For the first part we can only accept a line segment where the delta is
	// +/-1 (horizontal) or +/- MAX_X (vertical)
	const deltaX = end.x - start.x;
	const deltaY = end.y - start.y;
	if (deltaX === 0 && deltaY === 0) {
		// "Point"
		throw new Error(
			`Invalid line segment ${toString(start)} -> ${toString(end)}: point`
		);
	} else if (deltaX !== 0 && deltaY !== 0) {
		// Non-trivial line
		console.debug(
			`Ignoring non-trivial line segment ${toString(start)} -> ${toString(
				end
			)}`
		);
		return false;
	} else {
		console.debug(
			`Applying line segment ${toString(start)} -> ${toString(end)}`
		);
	}

	// "Draw" the line
	const startIndex = start.y * maxX + start.x;
	const endIndex = end.y * maxX + end.x;
	let deltaIndex;
	if (deltaX === 0) {
		deltaIndex = deltaY < 0 ? -maxX : maxX;
	} else {
		deltaIndex = deltaX < 0 ? -1 : 1;
	}
	let index = startIndex;
	do {
		floor.set(index, (floor.get(index) ?? 0) + 1);
		if (index === endIndex) {
			break;
		}
		index += deltaIndex;
	} while (true);
	return true;
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	const opts: FloorOptions = input.includes('example')
		? { maxX: 10, maxY: 10 }
		: { maxX: 1000, maxY: 1000 };
	return new Promise((resolve, reject) => {
		const floor: Floor = new Map();
		rl.on('line', (line) => {
			const lineSegment = parseLineSegment(line);
			if (updateFloor(floor, lineSegment, opts)) {
				// printFloor(floor, opts);
			}
		});
		rl.on('error', (err) => {
			reject(err);
		});
		rl.on('close', () => {
			const overlappingPoints = Array.from(floor.values()).reduce(
				(s, v) => (v > 1 ? s + 1 : s),
				0
			);
			console.log(`Results for ${input}: ${overlappingPoints}`);

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
