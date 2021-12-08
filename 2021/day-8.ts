#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	return new Promise((resolve, reject) => {
		let count1478 = 0;

		rl.on('line', (line) => {
			const outputDigits = line.split(/\|/)[1].split(/\s+/);
			for (const digit of outputDigits) {
				if ([2, 4, 3, 7].includes(digit.length)) {
					count1478++;
				}
			}
		});
		rl.on('error', (err) => {
			reject(err);
		});
		rl.on('close', () => {
			console.log(`Results for ${input}: ${count1478}`);

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
