#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

function processInput(input: string) {
	const rl = createInterface(createReadStream(input));

	const windowSize = 3;
	const window: number[] = [];
	let increases: number = 0;
	rl.on('line', line => {
		const value = Number(line);
		let lastSum;
		if (window.length === windowSize) {
			lastSum = window.reduce((s, v) => s + v, 0);
			window.splice(0, 1);
		}
		window.push(value);
		if (typeof lastSum !== 'undefined') {
			// We removed one, and we added one -- so window.length is still equal to the window size.
			const sum = window.reduce((s, v) => s + v, 0);
			if (sum > lastSum) {
				increases++;
			}
		}
	});
	rl.on('close', () => {
		console.log(`Results for ${input}: ${increases} increases`);
	});
}

const INPUT_SPECS = [
	'-example',
	'',
];
for (const inputSpec of INPUT_SPECS) {
	const inputFile = `${basename(process.argv[1], extname(process.argv[1]))}${inputSpec}.input`;
	processInput(inputFile);
}
