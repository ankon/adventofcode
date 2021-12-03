#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

/**
 * Complete report
 *
 * - String values must have the same length
 * - String values must only contain '0' and '1' characters
 */
type Report = string[];
type Counter = [zeros: number, ones: number];
type Counters = Counter[];

function selectMostCommon([zeros, ones]: Counter, ifEqual: number) {
	if (zeros === ones) {
		return ifEqual;
	} else {
		return zeros > ones ? 0 : 1;
	}
}

function selectLeastCommon([zeros, ones]: Counter, ifEqual: number) {
	if (zeros === ones) {
		return ifEqual;
	} else {
		return zeros > ones ? 1 : 0;
	}
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

function toCounters(report: Report): Counters {
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


/**
 * Filter the report at `position` using `select` and return a new report
 *
 * @param report
 * @param counters
 * @param position
 */
 function filterReport(report: Report, counters: Counters, position: number, select: (counter: Counter) => number): Report {
	// NB: Counters are LSB->MSB, but position is MSB->LSB
	const counter = counters[counters.length - position - 1];
	// Select the value to keep
	const bit = select(counter);
	const result: Report = [];
	for (let i = 0; i < report.length; i++) {
		const value = report[i];
		if (value[position] === `${bit}`) {
			result.push(value);
		}
	}

	return result;
}

function createLifeSupportRating(report: Report, select: (counter: Counter) => number): number {
	if (report.length === 0) {
		throw new Error('Report is empty');
	}
	const bits = report[0].length;

	let position = 0;
	let workReport = [...report];
	while (workReport.length !== 1 && position < bits) {
		const counters = toCounters(workReport);
		workReport = filterReport(workReport, counters, position, select);
		position++;
	}

	if (workReport.length === 1) {
		return parseInt(workReport[0], 2);
	} else if (workReport.length === 0) {
		throw new Error('No matching numbers');
	} else {
		throw new Error('Too many matching numbers');
	}
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	return new Promise((resolve, reject) => {
		const report: Report = [];

		rl.on('line', line => {
			report.push(line);
		});
		rl.on('error', err => {
			reject(err);
		});
		rl.on('close', () => {
			const counters = toCounters(report);
            // NB: For the power rates the `ifEqual` case in the selector should not happen, the `-1` will trigger an error.
			const gammaRate = createPowerRate(counters, counter => selectMostCommon(counter, -1));
			const epsilonRate = createPowerRate(counters, counter => selectLeastCommon(counter, -1));
			const oxygenGeneratorRating = createLifeSupportRating(report, counter => selectMostCommon(counter, 1));
			const co2ScrubberRating = createLifeSupportRating(report, counter => selectLeastCommon(counter, 0));
			console.log(`Results for ${input}: power consumption = ${gammaRate} * ${epsilonRate} = ${gammaRate * epsilonRate}, life support rating = ${oxygenGeneratorRating} * ${co2ScrubberRating} = ${oxygenGeneratorRating * co2ScrubberRating}`);

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
