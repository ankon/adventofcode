#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

type Counter = [zeros: number, ones: number];
type Counters = Counter[];

function MostCommon([zeros, ones]: Counter) {
	// XXX: What if the numbers are the same?
	return zeros > ones ? 0 : 1;
}
function LeastCommon([zeros, ones]: Counter) {
	// XXX: What if the numbers are the same?
	return zeros > ones ? 1 : 0;
}

function createPowerRate(counters: Counters, select: (counter: Counter) => number): number {
	let result = 0;
	let shift = 0;
	for (let i = 0; i < counters.length; i++) {
		const bit = select(counters[i]);
		result |= (bit << shift);
		shift++;
	}
	return result;
}

function updateCounters(counters: Counters, value: string) {
	// Value is a string of '0'/'1' characters, exactly as many as there are counters
	// (otherwise we would have to apply magic to backfill them if the length changes, which our
	// input doesn't have.)
	Array.from(value).forEach((c: string, index) => {
		if (c !== '0' && c !== '1') {
			throw new Error(`Not a valid value: ${value}`);
		}
		counters[value.length - index - 1][Number(c)]++;
	});
}

function toCounters(report: string[]): Counters {
	const counters: Counters = [];

	for (const value of report) {
		// Initialize on the first value
		if (counters.length === 0) {
			for (let i = 0; i < value.length; i++) {
				counters.push([0, 0]);
			}
		} else if (counters.length !== value.length) {
			throw new Error(`Change in length not supported`);
		}

		updateCounters(counters, value);
	}
	return counters;
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	return new Promise((resolve, reject) => {
		const report: string[] = [];

		rl.on('line', line => {
			report.push(line);
		});
		rl.on('error', err => {
			reject(err);
		});
		rl.on('close', () => {
			const counters = toCounters(report);
			const gammaRate = createPowerRate(counters, MostCommon);
			const epsilonRate = createPowerRate(counters, LeastCommon);
			console.log(`Results for ${input}: power consumption = ${gammaRate} * ${epsilonRate} = ${gammaRate * epsilonRate}`);

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
