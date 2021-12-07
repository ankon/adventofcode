#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

function alignCrabs(positions: number[]): number {
	// Brute-force and find the position that takes the least amount of
	// fuel.
	const maxPosition = positions.reduce((max, p) => Math.max(max, p), 0);
	let lowestCost = Number.POSITIVE_INFINITY;
	position: for (let candidate = 0; candidate < maxPosition; candidate++) {
		let cost = 0;
		for (let crab = 0; crab < positions.length; crab++) {
			cost += Math.abs(candidate - positions[crab]);
			if (cost > lowestCost) {
				continue position;
			}
		}
		lowestCost = cost;
	}

	return lowestCost;
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	return new Promise((resolve, reject) => {
		let positions: number[];

		rl.on('line', (line) => {
			positions = line.split(/,/).map((s) => parseInt(s, 10));
		});
		rl.on('error', (err) => {
			reject(err);
		});
		rl.on('close', () => {
			console.log(`Results for ${input}: ${alignCrabs(positions)}`);
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
