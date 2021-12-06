#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

function simulate(ages: number[], days: number): number {
	let state = ages;
	for (let i = 0; i < days; i++) {
		state = state.reduce((r, age) => {
			if (age === 0) {
				// Produce new fish
				r.push(6);
				r.push(8);
			} else {
				// Count down
				r.push(age - 1);
			}
			return r;
		}, [] as number[]);
	}
	return state.length;
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	/** Days to simulate */
	const DAYS = 80;

	/** Ages of all fish ("state") */
	let ages: number[];

	return new Promise((resolve, reject) => {
		rl.on('line', line => {
			ages = line.split(/,/).map((v) => parseInt(v, 10));
		});
		rl.on('error', err => {
			reject(err);
		});
		rl.on('close', () => {
			const fishAtEnd = simulate(ages, DAYS);
			console.log(`Results for ${input}: ${fishAtEnd} fish after ${DAYS} days`);

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
	'-example',
	'',
];

main(INPUT_SPECS.map(inputSpec => `${basename(process.argv[1], extname(process.argv[1]))}${inputSpec}.input`)).catch(err => console.error(err));
