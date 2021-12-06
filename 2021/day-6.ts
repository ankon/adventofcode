#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

/**
 * Simulate the fish "naively"
 *
 * @param ages
 * @param days
 * @returns
 */
function simulateBruteForce(ages: number[], days: number): number {
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

/**
 * Simulate the fish using age buckets
 *
 * @param ages
 * @param days
 */
function simulateWithBuckets(ages: number[], days: number): number {
	/** Age buckets: Number of fish per age group */
	const buckets: number[] = ages.reduce((r, age) => {
		r[age]++;
		return r;
	}, new Array(9).fill(0));

	for (let i = 0; i < days; i++) {
		// Shift the array left, and create new fish for the number of fish
		// on age 0
		const [newParents] = buckets.splice(0, 1);
		// Parents start back at 6
		buckets[6] += newParents;
		// ... and the newly spawned fish pop up at the end (which we just shifted away, so we
		// just need to push that back there).
		buckets.push(newParents);
	}

	// Add up all the counts to come to the number of fish
	return buckets.reduce((r, count) => r + count, 0);
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

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
			// XXX: Use both functions to not get complaints about unused code
			const fishAtEndPart1 = simulateBruteForce(ages, 80);
			const fishAtEndPart2 = simulateWithBuckets(ages, 256);
			console.log(`Results for ${input}: ${fishAtEndPart1} fish after 80 days, ${fishAtEndPart2} after 256 days`);

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
