#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

function printField(field: number[][]) {
	field.forEach((row) => console.log(row.join('')));
	console.log();
}

function simulateStep(field: number[][]): number[][] {
	const newField: number[][] = [];
	/** To-Do list of [row, col] octopi that must still flash in this step */
	const mustFlash: [number, number][] = [];
	for (let j = 0; j < field.length; j++) {
		const row = field[j];
		const newRow = new Array(row.length);
		for (let i = 0; i < row.length; i++) {
			newRow[i] = row[i] + 1;
			if (newRow[i] > 9) {
				mustFlash.push([j, i]);
			}
		}
		newField.push(newRow);
	}

	// Process flashes until there are none left
	// Order doesn't really matter, so we just use a stack.
	let nextFlash: [number, number] | undefined;
	while ((nextFlash = mustFlash.pop())) {
		// Bump all neighbors of this octopus in their energy level
		// and put any of them that reached 10 onto the todo list
		for (
			let nj = Math.max(nextFlash[0] - 1, 0);
			nj <= Math.min(nextFlash[0] + 1, newField.length - 1);
			nj++
		) {
			const row = newField[nj];
			for (
				let ni = Math.max(nextFlash[1] - 1, 0);
				ni <= Math.min(nextFlash[1] + 1, row.length - 1);
				ni++
			) {
				if (nj == nextFlash[0] && ni === nextFlash[1]) {
					continue;
				}

				if (++row[ni] === 10) {
					mustFlash.push([nj, ni]);
				}
			}
		}
	}

	// Reset the energy of all octopi > 9 to back to 0.
	// The number of these resets represents the number of flashes
	// that have happened (which we let the caller calculate).
	for (let j = 0; j < newField.length; j++) {
		const row = newField[j];
		for (let i = 0; i < row.length; i++) {
			if (row[i] > 9) {
				row[i] = 0;
			}
		}
	}
	return newField;
}

function simulate(field: number[][], steps: number = 100): number {
	console.log(`Before any steps:`);
	printField(field);
	let workField = field;
	let flashes = 0;
	for (let step = 1; step <= steps; step++) {
		workField = simulateStep(workField);

		// A octopus with energy 0 has just flashed, so count those.
		const newFlashes = workField.reduce(
			(result, row) =>
				result +
				row.reduce(
					(rowCount, e) => (e === 0 ? rowCount + 1 : rowCount),
					0
				),
			0
		);
		if (newFlashes === workField.length * workField[0].length) {
			console.log(`Step ${step}: Everyone flashed!`);
			printField(workField);
			break;
		}
		flashes += newFlashes;

		if (step < 10 || step % 10 === 0) {
			console.log(`After step ${step}:`);
			printField(workField);
			//console.log(`... ${flashes} flashes seen`);
		}
	}

	return flashes;
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	return new Promise((resolve, reject) => {
		const field: number[][] = [];

		rl.on('line', (line) => {
			field.push(Array.from(line.trim()).map((c) => parseInt(c, 10)));
		});
		rl.on('error', (err) => {
			reject(err);
		});
		rl.on('close', () => {
			const flashes = simulate(field, 100);
			console.log(`Results for ${input}: ${flashes} flashes`);
			simulate(field, 5000);
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
	//'-example-mini',
	//'-example',
	'',
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
