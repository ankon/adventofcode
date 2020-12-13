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

type GetNeighborOccupancy = (field: State[][], i: number, j: number) => number;

function getNeighborOccupancyPart1(field: State[][], i: number, j: number): number {
	return adjacency.reduce((result, [di, dj]) => {
		const newI = i + di;
		const newJ = j + dj;
		if (between(newI, 0, field.length - 1) && between(newJ, 0, field[i].length - 1)) {
			return result + (field[newI][newJ] === '#' ? 1 : 0);
		} else {
			return result;
		}
	}, 0);
}

function getNeighborOccupancyPart2(field: State[][], i: number, j: number): number {
	let count = 0;
	for (const [di, dj] of adjacency) {
		let ni = i + di, nj = j + dj;
		while (between(ni, 0, field.length - 1) && between(nj, 0, field[ni].length - 1) && field[ni][nj] === '.') {
			ni += di;
			nj += dj;
		}

		if (between(ni, 0, field.length - 1) && between(nj, 0, field[ni].length - 1) && field[ni][nj] === '#') {
			count++;
		}
	}
	return count;
}

function iterate(field: State[][], getNeighborOccupancy: GetNeighborOccupancy, dieAtOrAbove: number): { newField: State[][], changes: number } {
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
			const occupied = getNeighborOccupancy(field, i, j);
			if (current === 'L' && occupied == 0) {
				newRow.push('#');
				changes++;
			} else if (current === '#' && occupied >= dieAtOrAbove) {
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

function logField(field: State[][]) {
	field.forEach(row => console.log(row.join('')));
}

function processInput(input: string) {
	const rl = createInterface(createReadStream(input));

	const initialField: State[][] = [];
	const cases = [
		{ getNeighborOccupancy: getNeighborOccupancyPart1, dieAtOrAbove: 4 },
		{ getNeighborOccupancy: getNeighborOccupancyPart2, dieAtOrAbove: 5 },
	];

	rl.on('line', line => {
		// Process `line` here
		initialField.push(Array.from(line) as State[]);
	});
	rl.on('close', () => {
		console.log(`Results for ${input}:`);
		for (const {getNeighborOccupancy, dieAtOrAbove} of cases) {
			let round = 1;
			let field = initialField;
			let result;
			logField(initialField);
			do {
				result = iterate(field, getNeighborOccupancy, dieAtOrAbove);
				field = result.newField;
				logField(field);
				console.log(`Changes after round ${round}: ${result.changes}`);
				round++;
			} while (result.changes > 0);

			// Count the number of occupied seats
			const seats = field.map(row => row.reduce((cnt, state) => cnt + (state === '#' ? 1 : 0), 0)).reduce((cnt, rowCount) => cnt + rowCount, 0);
			console.log(`Occupied ${getNeighborOccupancy.name}: ${seats}`);
		}
	});
}

processInput(`${basename(process.argv[1], extname(process.argv[1]))}-example.input`);
processInput(`${basename(process.argv[1], extname(process.argv[1]))}.input`);
