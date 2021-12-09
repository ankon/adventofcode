#!/usr/bin/env node

import { createReadStream, writeFileSync } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

type Map = number[][];
type Point = [row: number, column: number];

function findNeighbors(
	map: Map,
	[rowIndex, colIndex]: Point,
	select: (height: number, otherHeight: number) => boolean
): Point[] {
	const result: Point[] = [];

	const height = map[rowIndex][colIndex];

	if (colIndex > 0 && select(height, map[rowIndex][colIndex - 1])) {
		result.push([rowIndex, colIndex - 1]);
	}
	if (
		colIndex < map[rowIndex].length - 1 &&
		select(height, map[rowIndex][colIndex + 1])
	) {
		result.push([rowIndex, colIndex + 1]);
	}
	if (rowIndex > 0 && select(height, map[rowIndex - 1][colIndex])) {
		result.push([rowIndex - 1, colIndex]);
	}
	if (
		rowIndex < map.length - 1 &&
		select(height, map[rowIndex + 1][colIndex])
	) {
		result.push([rowIndex + 1, colIndex]);
	}
	return result;
}

function colorBasin(
	map: Map,
	coloredMap: Map,
	openPoints: Point[],
	basin: number
): number {
	if (openPoints.length === 0) {
		return 0;
	}

	// Color the points with the basin number, and enumerate the neighbors that are same height or higher, but not 9.
	function isHigherOrEqualButNot9(height: number, otherHeight: number) {
		return otherHeight < 9 && otherHeight >= height;
	}

	let size = 0;
	while (openPoints.length > 0) {
		const point = openPoints.pop()!;
		const existingBasin = coloredMap[point[0]][point[1]];
		if (existingBasin !== -1) {
			// Ignore this point, we've already been there.
			if (existingBasin !== basin) {
				throw new Error(
					`Spilled into another basin (found ${existingBasin} while processing ${basin})`
				);
			}
			continue;
		}

		coloredMap[point[0]][point[1]] = basin;
		size++;

		for (const neighbor of findNeighbors(
			map,
			point,
			isHigherOrEqualButNot9
		)) {
			const existingBasin = coloredMap[neighbor[0]][neighbor[1]];
			if (existingBasin === basin) {
				// Fine, ignore.
				continue;
			} else if (existingBasin === -1) {
				// Could be interesting to look at.
				openPoints.push(neighbor);
			} else {
				// Very much not good: We are spilling into another basin?
				throw new Error(
					`Would spill into another basin (found ${existingBasin} while processing ${basin})`
				);
			}
		}
	}

	return size;
}

function findTop3BasinSizes(map: Map, lowPoints: Point[]): number[] {
	// Finding the basins: Start at each low point, and assign a basin number. Then walk upwards around it, and color each point with the basin
	// number. Finally, count how many points are assigned to each basin, and pick the top 3.
	const basinSizes: number[] = [];
	const coloredMap: Map = map.map((row) => new Array(row.length).fill(-1));
	for (let i = 0; i < lowPoints.length; i++) {
		const size = colorBasin(map, coloredMap, [lowPoints[i]], i);
		basinSizes.push(size);
	}

	basinSizes.sort((a, b) => a - b);
	return basinSizes.slice(-3);
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	return new Promise((resolve, reject) => {
		const map: Map = [];

		rl.on('line', (line) => {
			map.push(line.split('').map((c) => parseInt(c, 10)));
		});
		rl.on('error', (err) => {
			reject(err);
		});
		rl.on('close', () => {
			const colorClasses = `
			.h4 { color: #ff4000ff; }
			.h3 { color: #fd7547ff; }
			.h2 { color: #fc906aff; }
			.h1 { color: #faaa8dff; }
			.h0 { color: #201e1fff; }
			`;

			// Walk over all elements, and check that that there is a smaller neighbor
			// somewhere. If there isn't, remember it as a low point.
			const debugMap: string[] = [];
			const lowestPointHeights: number[] = [];
			const lowPoints: Point[] = [];
			for (let rowIndex = 0; rowIndex < map.length; rowIndex++) {
				let debugLine = '';
				const row = map[rowIndex];
				for (let colIndex = 0; colIndex < row.length; colIndex++) {
					// XXX: The task isn't exactly clear, but it seems we're only a low point
					//      if none of our neighbors is at the same height or higher (i.e. plateaus don't count?)
					function isLower(height: number, otherHeight: number) {
						return otherHeight <= height;
					}

					const lowerNeighbors = findNeighbors(
						map,
						[rowIndex, colIndex],
						isLower
					).length;
					let text;
					const height = row[colIndex];
					if (lowerNeighbors === 0) {
						lowestPointHeights.push(height);
						lowPoints.push([rowIndex, colIndex]);
						text = `<strong>${height}</strong>`;
					} else {
						text = `${height}`;
					}
					debugLine += `<span class="h${lowerNeighbors}">${text}</span>`;
				}
				debugMap.push(debugLine);
			}
			const totalRiskLevel = lowestPointHeights
				.map((height) => height + 1)
				.reduce((s, riskLevel) => s + riskLevel, 0);
			const top3BasinSizes = findTop3BasinSizes(map, lowPoints);

			console.log(
				`Results for ${input}: total risk level ${totalRiskLevel}, basin indicator ${top3BasinSizes.reduce(
					(p, s) => p * s,
					1
				)}`
			);

			writeFileSync(
				'day-9-debug.html',
				`<!DOCTYPE html><html><head><style>${colorClasses}</style></head><body><pre>${debugMap.join(
					'\n'
				)}</pre></body></html>`,
				'utf-8'
			);
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
	'-example',
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
