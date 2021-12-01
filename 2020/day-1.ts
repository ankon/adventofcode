#!/usr/bin/env node

import { createInterface } from 'readline';
import { createReadStream } from 'fs';

import { qs } from '../utils';

function part1BruteForce(numbers: number[]): [number, number, number] {
	for (let i = 0; i < numbers.length; i++) {
		for (let j = 0; j < numbers.length; j++) {
			if (i === j) {
				continue
			} else if (numbers[i] + numbers[j] === 2020) {
				const product = numbers[i] * numbers[j];
				console.log(`Brute force: ${numbers[i]} + ${numbers[j]} = 2020; product: ${product}`);
				return [numbers[i], numbers[j], product];
			}
		}
	}
	throw new Error('Not found!');
}

function part1IndexOf(numbers: number[], sum: number = 2020, indexBlacklist: number[] = []): [number, number, number] {
	for (let i = 0; i < numbers.length; i++) {
		if (indexBlacklist.includes(i)) {
			continue;
		}
		const number = numbers[i];
		const needed = sum - number;
		const neededIndex = qs(numbers, needed);
		if (neededIndex >= 0 && neededIndex !== i) {
			const product = needed * number;
			console.log(`indexOf: ${number} + ${needed} = ${sum}; product: ${product}`);
			return [number, needed, product];
		}
	}
	throw new Error('Not found');
}

function part2IndexOf(numbers: number[]): [number, number, number, number] {
	for (let i = 0; i < numbers.length; i++) {
		const number = numbers[i];
		const needed = 2020 - number;
		try {
			const [number2, number3, partProduct] = part1IndexOf(numbers, needed, [i]);
			const product = number * partProduct;
			console.log(`part 2: indexOf: ${number} + ${number2} + ${number3} = 2020; product: ${product}`);
			return [number, number2, number, product];
		} catch (err) {
			// Ignored
		}
	}
	throw new Error('Not found');
}

const numbers: number[] = [];

// Self-test
console.log(qs([0,1,2,3], 0));
console.log(qs([0,1,2,3], 1));
console.log(qs([0,1,2,3], 2));
console.log(qs([0,1,2,3], 3));
console.log(qs([0,1,2,3], 5));

const rl = createInterface(createReadStream('day-1.input'));
rl.on('line', line => {
	numbers.push(Number(line));
});
rl.on('close', () => {
	console.log(`Have ${numbers.length} numbers`);
	const sorted = numbers.sort();
	part1BruteForce(sorted);
	part1IndexOf(sorted);
	part2IndexOf(sorted);
});