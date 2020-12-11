#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

import { qs } from './utils';

function makeKey(queue: number[]): string {
	return queue.join('-');
}

function countArrangements(queue: number[], cache: Record<string, {value: number; hits: number}>, indent = ''): number {
	// We want to start at the first item, and see how many options we have
	// And then ... run through each of these
	const options: number[][] = [];
	const first = queue[0];
	for (let i = 1; i < queue.length; i++) {
		const second = queue[i];
		if (second - first > 3) {
			// Ok, this doesn't work anymore
			break;
		} else {
			// Possible option
			options.push([first, second, ...queue.slice(i + 1)]);
		}
	}
	// For each of these options we now know that the first two entries are fixed, and we
	// need to recurse on the third-and-following one
	let arrangements = 0;
	for (const option of options) {
		const newQueue = option.slice(1);
		if (newQueue.length < 2) {
			// Reached the end with this option
			arrangements++;
		} else {
			const key = makeKey(newQueue);
			const cachedArrangements = cache[key];
			if (typeof cachedArrangements !== 'undefined') {
				arrangements += cachedArrangements.value;
				cachedArrangements.hits++;
			} else {
				const newArrangements = countArrangements(newQueue, cache, indent + '  ');
				cache[key] = { value: newArrangements, hits: 0 };
				arrangements += newArrangements;
			}
		}
	}
	return arrangements;
}

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

		console.log(`Input: ${joltages}`);
		const cache: Record<string, {value: number, hits: number}> = {};
		const arrangements = countArrangements(joltages, cache);
		console.log(`Arrangements: ${arrangements}`);
		console.log(`Cache: ${JSON.stringify(cache, undefined, 2)}`);
	});
}

processInput(`${basename(process.argv[1], extname(process.argv[1]))}-example-1.input`);
processInput(`${basename(process.argv[1], extname(process.argv[1]))}-example-2.input`);
processInput(`${basename(process.argv[1], extname(process.argv[1]))}.input`);