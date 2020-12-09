#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

function isValid(window: number[], n: number): boolean {
	for (let i = 0; i < window.length; i++) {
		for (let j = 0; j < window.length; j++) {
			if (i === j) {
				continue;
			}
			if (window[i] + window[j] === n) {
				return true;
			}
		}
	}
	return false;
}

function findFirstInvalid(numbers: number[], windowLength: number): { index: number, value: number } {
	for (let index = windowLength; index < numbers.length; index++) {
		const window = numbers.slice(index - windowLength, index);
		if (!isValid(window, numbers[index])) {
			return { index, value: numbers[index] };
		}
	}
	throw new Error('All valid!');
}

function findWeakness(numbers: number[], value: number): { firstIndex: number, lastIndex: number, weakness: number } {
	// XXX: This really smells like a faster string-search algorithm, dynamic programming or such similar to levenshtein, might work
	//      quite well here.
	for (let firstIndex = 0; firstIndex < numbers.length; firstIndex++) {
		// Try summing from here until we find a value bigger than number
		const firstValue = numbers[firstIndex];
		let sum = firstValue;
		let lastIndex = firstIndex;
		let smallest = firstValue;
		let largest = firstValue;
		do {
			const v = numbers[++lastIndex];
			sum += v;
			if (v < smallest) {
				smallest = v;
			} else if (v > largest) {
				largest = v;
			}
		} while (sum < value);
		if (sum === value) {
			return { firstIndex, lastIndex, weakness: smallest + largest };
		}
	}
	throw new Error('Cannot find weakness');
}

function processInput(input: string, preamble: number) {
	const rl = createInterface(createReadStream(input));

	// Collect everything, rather than immediately processing things to simplify
	// the code to drive readline.
	const numbers: number[] = [];
	rl.on('line', line => {
		const n = Number(line);
		numbers.push(n);
	});
	rl.on('close', () => {
		// Report results for this input
		console.log(`Results for ${input}:`);
		const { index, value } = findFirstInvalid(numbers, preamble);
		console.log(`Entry at position ${index} is invalid: ${value}`);

		const { firstIndex, lastIndex, weakness } = findWeakness(numbers, value);
		console.log(`Entries between ${firstIndex} and ${lastIndex} (${numbers.slice(firstIndex, lastIndex + 1)}) form the weakness of ${weakness}`);
	});
}

processInput(`${basename(process.argv[1], extname(process.argv[1]))}-example.input`, 5);
processInput(`${basename(process.argv[1], extname(process.argv[1]))}.input`, 25);