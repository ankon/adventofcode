#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

import { qs } from './utils';

function processInput(input: string) {
	const rl = createInterface(createReadStream(input));

	// Start with the outlet (joltage of 0)
	const joltages: number[] = [0];

	rl.on('line', line => {
		const joltage = Number(line);
		const insertAt = -(qs(joltages, joltage) + 1);
		joltages.splice(insertAt, 0, joltage);
	});
	rl.on('close', () => {
		console.log(`Results for ${input}:`);

		// Add the actual device adapter
		joltages.push(joltages[joltages.length - 1] + 3);

		// Adapters need to fit, and we need to use all ... so this is merely a question
		// of what are the differences between these values.
		const diffs: number[] = [];
		let diffOne = 0;
		let diffThree = 0;
		for (let i = 1; i < joltages.length; i++) {
			const diff = joltages[i] - joltages[i - 1];
			diffs.push(diff);
			if (diff === 1) {
				diffOne++;
			} else if (diff === 3) {
				diffThree++;
			} else {
				throw new Error(`Found an unexpected diff of ${diff}`);
			}
		}
		console.log(`${diffOne} of 1, ${diffThree} of 3: ${diffOne * diffThree} [${diffs}]`);

		// A (1,1) can be a single (2) (adding one extra arrangement)
		// A (1,2) can be a single (3) (adding one extra arrangement)
		// A (2,1) can be a single (3) (adding one extra arrangement)
		let arrangements = 1;
		let delta = 0;
		let count = 0;
		let runStart = -1;
		for (let i = 0; i < diffs.length; i++) {
			const diff = diffs[i];
			if (diff === 1) {
				if (runStart === -1) {
					// New run
					runStart = i;
					delta = 1;
					count = 1;
				} else if (delta + diff <= 3) {
					// Existing run, and we can add to it
					delta++;
					count++;
				} else {
					runStart++;
					arrangements *= 2 ** (count - 1);
				}
			} else {
				if (count > 1) {
					arrangements *= 2 ** (count - 1);
				}
				count = 0;
				runStart = -1;
			}
		}
		console.log(`Arrangements: ${arrangements}`);
	});
}

processInput(`${basename(process.argv[1], extname(process.argv[1]))}-example-1.input`);
processInput(`${basename(process.argv[1], extname(process.argv[1]))}-example-2.input`);
//processInput(`${basename(process.argv[1], extname(process.argv[1]))}.input`);