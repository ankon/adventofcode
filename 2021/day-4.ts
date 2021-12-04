#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

type Board = number[][];

function parseBoard(lines: string[]): Board {
	const result: Board = [];
	for (const line of lines) {
		const row = line
			.trim()
			.split(/\s+/)
			.map((s) => parseInt(s, 10));
		if (row.length !== lines.length) {
			throw new Error(
				`Expected row with ${lines.length} numbers, got "${line}"`
			);
		}
		result.push(row);
	}
	return result;
}

function printBoard(board: Board, marks?: boolean[][]) {
	for (let rowIndex in board) {
		const row = board[rowIndex];
		console.log(
			row
				.map((n, columnIndex) => {
					const symbol = marks?.[rowIndex][columnIndex] ? 'X' : n;
					return String(symbol).padStart(4);
				})
				.join(' ')
		);
	}
	console.log();
}

function updateMarks(n: number, board: Board, marks: boolean[][]): boolean {
	// Walk over all elements of the board, update the marks, and return whether
	// bingo was achieved.
	let result = false;
	const columnBingos: boolean[] = new Array(board.length).fill(true);
	for (let rowIndex in board) {
		let rowBingo = true;
		for (let columnIndex in board) {
			const at = board[rowIndex][columnIndex];
			if (at === n) {
				marks[rowIndex][columnIndex] = true;
			}
			rowBingo &&= marks[rowIndex][columnIndex];
			columnBingos[columnIndex] &&= marks[rowIndex][columnIndex];
		}
		if (rowBingo) {
			result = true;
		}
	}

	return result || columnBingos.includes(true);
}

function sumUnmarked(board: Board, marks: boolean[][]): number {
	let result: number = 0;
	for (let rowIndex in board) {
		for (let columnIndex in board) {
			if (!marks[rowIndex][columnIndex]) {
				result += board[rowIndex][columnIndex];
			}
		}
	}

	return result;
}

function playBingo(card: Board[], numbers: number[]): number {
	// Prepare marks for each board
	const boardMarks: boolean[][][] = [];
	for (const board of card) {
		const marks: boolean[][] = [];
		for (let i = 0; i < board.length; i++) {
			marks.push(new Array(board.length).fill(false));
		}
		boardMarks.push(marks);
	}

	// Process the numbers until we see a bingo in a board
	for (const n of numbers) {
		for (const boardIndex in card) {
			// if (boardIndex === '2') {
			// 	printBoard(card[boardIndex], boardMarks[boardIndex]);
			// }
			if (updateMarks(n, card[boardIndex], boardMarks[boardIndex])) {
				return (
					n * sumUnmarked(card[boardIndex], boardMarks[boardIndex])
				);
			}
		}
	}

	throw new Error('No more numbers, no winner!');
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	return new Promise((resolve, reject) => {
		const sequence: number[] = [];
		const card: Board[] = [];
		const pendingLines: string[] = [];

		rl.on('line', (line) => {
			// Process `line` here
			if (sequence.length === 0) {
				sequence.push(
					...line
						.trim()
						.split(/,/)
						.map((s) => parseInt(s, 10))
				);
			} else if (line) {
				pendingLines.push(line);
			} else if (pendingLines.length > 0) {
				card.push(parseBoard(pendingLines.splice(0)));
			}
		});
		rl.on('error', (err) => {
			reject(err);
		});
		rl.on('close', () => {
			if (pendingLines.length > 0) {
				card.push(parseBoard(pendingLines.splice(0)));
			}
			const result = playBingo(card, sequence);
			console.log(`Results for ${input}: ${result}`);

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
