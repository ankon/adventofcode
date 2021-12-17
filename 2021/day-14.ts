#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

type Rules = Record<string, string>;

/** Counter for each pair how often it is known to exist */
type State = Record<string, number>;

function processState(state: State, rules: Rules): State {
	const result: State = {};
	for (const [pair, count] of Object.entries(state)) {
		const insertion = rules[pair];
		if (insertion) {
			const pair1 = pair.charAt(0) + insertion;
			const pair2 = insertion + pair.charAt(1);
			result[pair1] = (result[pair1] ?? 0) + count;
			result[pair2] = (result[pair2] ?? 0) + count;
		} else {
			result[pair] = (result[pair] ?? 0) + count;
		}
	}
	return result;
}

function processTemplate(
	template: string,
	rules: Rules,
	iterations: number
): number {
	// Basic approach: We keep track of pairs and how often they exist in the string
	// When processing we look at each pair. Either it matches a rule, in which case it produces (up to) two pairs, otherwise it gets copied literally.
	// When counting characters we walk over all pairs, and count just the first character. As the last character of the input would not be recognizable
	// then, we introduce an additional character at the end to make a pair out of that, too.
	// Alternative approach: The last character will never change, so we could also simply bump the final counters by one for the last character.

	let state: State = {};

	// Step 1: Split the initial template into a counter for the pairs
	const templateWithEnd = `${template} `;
	for (let i = 0; i < templateWithEnd.length - 1; i++) {
		const pair = templateWithEnd.slice(i, i + 2);
		state[pair] = (state[pair] ?? 0) + 1;
	}

	// Step 2: Process the state for the given number of iterations
	for (let i = 0; i < iterations; i++) {
		const newState = processState(state, rules);
		state = newState;
	}

	// Step 3: Count the number of characters, and pick the lowest and highest
	const counters: Record<string, number> = {};
	for (const [pair, count] of Object.entries(state)) {
		const c = pair.charAt(0);
		counters[c] = (counters[c] ?? 0) + count;
	}
	const [maxCount, minCount] = Object.entries(counters).reduce(
		([max, min], [_, count]) => [
			Math.max(max, count),
			Math.min(min, count),
		],

		[0, Number.MAX_SAFE_INTEGER]
	);
	return maxCount - minCount;
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	const rules: Rules = {};
	let template: string;

	return new Promise((resolve, reject) => {
		rl.on('line', (line) => {
			if (!line) {
				return;
			}

			const [pair, insert] = line.split(/ -> /);
			if (!insert) {
				template = line;
			} else {
				rules[pair] = insert;
			}
		});
		rl.on('error', (err) => {
			reject(err);
		});
		rl.on('close', () => {
			const indicatorPart1 = processTemplate(template, rules, 10);
			const indicatorPart2 = processTemplate(template, rules, 40);
			console.log(
				`Results for ${input}: after 10 iterations = ${indicatorPart1}, after 40 iterations = ${indicatorPart2}`
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
