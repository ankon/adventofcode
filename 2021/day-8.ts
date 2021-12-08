#!/usr/bin/env node

import { debug } from 'console';
import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

function split(s1: string, s2: string): [common: string, different: string] {
	// Join, and count: If a character appears 2 times its common, otherwise it's different.
	// This works because the alphabet is known and fairly small, so we don't bother too much here.
	let common: string = '';
	let different: string = '';
	const both = Array.from(s1 + s2);
	for (const c of 'abcdefg') {
		const count = both.reduce((s, x) => x === c ? s + 1 : s, 0);
		if (count === 1) {
			different += c;
		} else if (count === 2) {
			common += c;
		}
	}
	return [common, different];
}

function recoverOutputValue(sampleDigits: string[], outputDigits: string[], debugLog: (...args: any[]) => void): number {
	// Work out the various digits
	function pickAndRemove(candidatePredicate: (digit: string) => boolean): string {
		const candidates = sampleDigits.filter(candidatePredicate);
		if (candidates.length !== 1) {
			throw new Error(`Multiple candidates`);
		}
		const result = candidates[0];
		const index = sampleDigits.indexOf(result);
		sampleDigits.splice(index, 1);
		return result;
	}

	// 1 = cf
	const cf = pickAndRemove(digit => digit.length === 2);

	// 4 = cf + bd
	const cfbd = pickAndRemove(digit => digit.length === 4);

	// 7 = cf + a
	const cfa = pickAndRemove(digit => digit.length === 3)!;

	const a = Array.from(cfa).find(x => !cf.includes(x));
	debugLog(`Identified a: ${a}`);

	// 8 = abcdefg
	const abcdefg = pickAndRemove(digit => digit.length === 7)!;

	// 6 is the one that differs by one character from 8 - 1; that difference is f, and from that we learn c
	const abdeg = Array.from(abcdefg).filter(x => !cf.includes(x)).join('');
	let f;
	const abdefg = pickAndRemove(digit => {
		const [, different] = split(digit, abdeg);
		if (different.length !== 1) {
			return false;
		}
		f = different;
		return true;
	});
	debugLog(`Identified f: ${f}`);

	const c = cf[0] === f ? cf[1] : cf[0];
	debugLog(`Identified c: ${c}`);

	// 5 is the one that differs from 6 by one character, which is e
	let e;
	const abdfg = pickAndRemove(digit => {
		if (digit.length !== 5) {
			return false;
		}
		const [common, different] = split(digit, abdefg);
		if (common.length !== 5 || different.length !== 1) {
			return false;
		}
		e = different;
		return true;
	});
	debugLog(`Identified e: ${e}`);

	// 9 is 5 + c
	const tmp = abdfg + c;
	const abcdfg = pickAndRemove(digit => {
		const [, different] = split(digit, tmp);
		return different.length === 0;
	});

	// 0 is the one with length 6 that isn't 9 or 6
	const abcefg = pickAndRemove(digit => digit.length === 6);

	// 3 has a difference of 1 to 9, which is b
	let b;
	const acdfg = pickAndRemove(digit => {
		const [, different ] = split(digit, abcdfg);
		if (different.length !== 1) {
			return false;
		}
		b = different;
		return true;
	});

	// 2 is the one that is left over
	const acdefg = pickAndRemove(() => true);

	if (sampleDigits.length !== 0) {
		throw new Error(`Huh? Left over: ${sampleDigits.join(',')}`);
	}

	// Build the magic mapping
	// The keys are the inputs, but with the segments sorted, so that the lookup can just pick
	const mapping = new Map<string, number>();
	function setMapping(digit: string, value: number) {
		mapping.set(Array.from(digit).sort().join(''), value);
	}
	function getMapping(digit: string): number {
		return mapping.get(Array.from(digit).sort().join(''))!;
	}

	setMapping(abcefg, 0);
	setMapping(cf, 1);
	setMapping(acdefg, 2);
	setMapping(acdfg, 3);
	setMapping(cfbd, 4);
	setMapping(abdfg, 5);
	setMapping(abdefg, 6);
	setMapping(cfa, 7);
	setMapping(abcdefg, 8);
	setMapping(abcdfg, 9);
	if (mapping.size !== 10) {
		throw new Error(`Duplicate mappings`);
	}

	let result = 0;
	let power = 1;
	for (let i = outputDigits.length - 1; i >= 0; i--) {
		const digit = outputDigits[i];
		result += getMapping(digit)! * power;
		power *= 10;
	}
	return result;
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	const debugLog = process.env.DEBUG ? console.debug.bind(console) : () => { /* No-op */ };

	return new Promise((resolve, reject) => {
		let count1478 = 0;
		let sumOutputValues = 0;

		rl.on('line', (line) => {
			const [samples, output] = line.split(/\|/);
			const sampleDigits = samples.trim().split(/\s+/);
			const outputDigits = output.trim().split(/\s+/);
			for (const digit of outputDigits) {
				if ([2, 4, 3, 7].includes(digit.length)) {
					count1478++;
				}
			}
			sumOutputValues += recoverOutputValue(sampleDigits, outputDigits, debugLog);
		});
		rl.on('error', (err) => {
			reject(err);
		});
		rl.on('close', () => {
			console.log(`Results for ${input}: count1478 = ${count1478}, sum = ${sumOutputValues}`);

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
	//
	'-example',
	''
];

main(
	INPUT_SPECS.map(
		(inputSpec) =>
			`${basename(
				process.argv[1],
				extname(process.argv[1])
			)}${inputSpec}.input`
	)
).catch((err) => console.error(err));
