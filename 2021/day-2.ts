#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	return new Promise((resolve, reject) => {
		let horizontalPosition = 0;
		let depth = 0;

		rl.on('line', line => {
			const [command, arg] = line.split(/ /);
			const amount = Number(arg);
			switch (command) {
				case 'forward':
					horizontalPosition += amount;
					break;
				case 'down':
					depth += amount;
					break;
				case 'up':
					depth -= amount;
					break;
			}
		});
		rl.on('error', err => {
			reject(err);
		});
		rl.on('close', () => {
			console.log(`Results for ${input}: ${horizontalPosition} * ${depth} = ${horizontalPosition * depth}`);
			resolve();
		});
	});
}

async function main(inputFiles: string[]) {
	for (const inputFile of inputFiles) {
		await processInput(inputFile);
	}
}

const INPUT_SPECS = [
	'-example',
	'',
];

main(INPUT_SPECS.map(inputSpec => `${basename(process.argv[1], extname(process.argv[1]))}${inputSpec}.input`)).catch(err => console.error(err));

