#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';
import { between } from './utils';

type State = '.'|'L'|'#';

const adjacency = [
	[-1, -1], [-1, 0], [-1, 1],
	[ 0, -1],          [ 0, 1],
	[ 1, -1], [ 1, 0], [ 1, 1],
];

function iterate(field: State[][]): { newField: State[][], changes: number } {
	const newField: State[][] = [];
	let changes = 0;

	const rowLength = field[0].length;
	for (let i = 0; i < field.length; i++) {
		const newRow: State[] = [];
		for (let j = 0; j < rowLength; j++) {
			const current = field[i][j];
			if (current === '.') {
				newRow.push('.');
				continue;
			}
			const count = adjacency.reduce((result, [di, dj]) => {
				const newI = i + di;
				const newJ = j + dj;
				if (between(newI, 0, field.length - 1) && between(newJ, 0, rowLength - 1)) {
					return result + (field[newI][newJ] === '#' ? 1 : 0);
				} else {
					return result;
				}
			}, 0);
			if (current === 'L' && count == 0) {
				newRow.push('#');
				changes++;
			} else if (current === '#' && count >= 4) {
				newRow.push('L');
				changes++;
			} else {
				newRow.push(current);
			}
		}
		newField.push(newRow);
	}
	return {newField, changes};
}

function processInput(input: string) {
	const rl = createInterface(createReadStream(input));

	const initialField: State[][] = [];

	rl.on('line', line => {
		// Process `line` here
		initialField.push(Array.from(line) as State[]);
	});
	rl.on('close', () => {
		console.log(`Results for ${input}:`);
		let round = 1;
		let field = initialField;
		let result;
		do {
			result = iterate(field);
			field = result.newField;
			field.forEach(row => console.log(row.join('')));
			console.log(`Changes after round ${round}: ${result.changes}`);
			round++;
		} while (result.changes > 0);
		
		// Count the number of occupied seats
		const seats = field.map(row => row.reduce((cnt, state) => cnt + (state === '#' ? 1 : 0), 0)).reduce((cnt, rowCount) => cnt + rowCount, 0);
		console.log(`Occupied: ${seats}`);
	});
}

processInput(`${basename(process.argv[1], extname(process.argv[1]))}-example.input`);
processInput(`${basename(process.argv[1], extname(process.argv[1]))}.input`);
