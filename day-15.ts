#!/usr/bin/env node

function processInput(input: string, turns: number): number {
	const numbers = input.split(/,/).map(v => parseInt(v));

	// Turns are counted 1,2,3,...
	// That means the 2020th number spoken will happen in turn 2020.
	// In terms of remembering what was spoken: The theoretically highest value could be the number of turns (if
	// the value was spoken once in the beginning, and then on the last turn again), and therefore the number of
	// entries is also at the most the number of turns.
	// For the 30M case, and assuming a reasonable storage of numbers as 8 bytes (floating point double) we need
	// at most 240MiB memory ... that's ok.
	const lastSpokenAt: number[] = new Array(turns);
	numbers.forEach((n, i) => {
		lastSpokenAt[n] = i + 1;
	});

	// The next number spoken will be 0, as the one before that (numbers[numbers.length - 1])
	// will have been new.
	let spoken = 0;
	for (let i = numbers.length + 1; i < turns; i++) {
		if (i % 10000 === 0) {
			console.debug(`${input} turn ${i}: Spoken ${spoken}`);
		}
		const previousSpokenAt = lastSpokenAt[spoken];
		lastSpokenAt[spoken] = i;
		if (previousSpokenAt) {
			spoken = i - previousSpokenAt;
		} else {
			spoken = 0;
		}
	}

	return spoken;
}

const cases = {
	'0,3,6': [436, 175594],
	'1,3,2': [1, 2578],
	'2,1,3': [10, 3544142],
	'1,2,3': [27, 261214],
	'2,3,1': [78, 6895259],
	'3,2,1': [438, 18],
	'3,1,2': [1836, 362],
}

for (const [input, expectedSpoken] of Object.entries(cases)) {
	const turns = [2020, 30000000];
	for (let i = 0; i < turns.length; i++) {
		const spoken = processInput(input, turns[i]);
		if (spoken !== expectedSpoken[i]) {
			throw new Error(`Failed in case ${input} after ${turns[i]} turns, got ${spoken} but expected ${expectedSpoken[i]}`);
		} else {
			console.log(`Case ${input} after ${turns[i]} turns: ${spoken}`);
		}
	}
}

const input = '0,3,1,6,7,5';
const resultPart1 = processInput(input, 2020);
console.log(`Case ${input} after 2020 turns: ${resultPart1}`);
const resultPart2 = processInput(input, 30000000);
console.log(`Case ${input} after 30000000 turns: ${resultPart2}`);
