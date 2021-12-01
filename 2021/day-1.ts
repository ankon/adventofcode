#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

function processInput(input: string) {
	const rl = createInterface(createReadStream(input));

	let lastValue: number|undefined = undefined;
	let increases: number = 0;
	rl.on('line', line => {
		const value = Number(line);
		if (lastValue && value > lastValue) {
			increases++;
		}
		lastValue = value;
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
