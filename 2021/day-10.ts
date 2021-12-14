#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

const SYNTAX_ERROR_SCORE: Record<string, number> = {
	')': 3,
	']': 57,
	'}': 1197,
	'>': 25137,
};

const PAIR_MAPPING: Record<string, string> = {
	'(': ')',
	'[': ']',
	'{': '}',
	'<': '>',
};

/**
 * Check the line for syntax errors
 *
 * Returns an index into the line for the first syntax error, or `0` if the line is syntactically
 * fine, or `-1` when the line is incomplete.
 *
 * @param line
 */
function syntaxCheck(line: string): number {
	/** Stack of "open" things that need closing */
	const expectedNextClosing: string[] = [];
	for (let i = 0; i < line.length; i++) {
		const c = line[i];
		// If c is a closing symbol, then it must match to the one on the top of
		// the stack, otherwise the line is corrupt.
		const closingSymbol = PAIR_MAPPING[c];
		if (closingSymbol) {
			// c is opening
			expectedNextClosing.push(closingSymbol);
		} else {
			// c is already closing
			const expected = expectedNextClosing.pop();
			if (c === expected) {
				// Good, proceed.
			} else {
				// Not good, return an error.
				return SYNTAX_ERROR_SCORE[c];
			}
		}
	}

	if (expectedNextClosing.length === 0) {
		// All good, nothing open!
		return 0;
	} else {
		// Something is still open, so the line is incomplete.
		return -1;
	}
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	const completeLines: string[] = [];
	const incompleteLines: string[] = [];
	let syntaxErrorScore = 0;

	return new Promise((resolve, reject) => {
		rl.on('line', (line) => {
			// Process `line` here
			const s = syntaxCheck(line);
			switch (s) {
				case 0:
					completeLines.push(line);
					break;
				case -1:
					incompleteLines.push(line);
					break;
				default:
					syntaxErrorScore += s;
					break;
			}
		});
		rl.on('error', (err) => {
			reject(err);
		});
		rl.on('close', () => {
			console.log(
				`Results for ${input}: syntax error score ${syntaxErrorScore}`
			);
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
