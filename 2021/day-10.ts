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

const AUTOCOMPLETE_SCORE: Record<string, number> = {
	')': 1,
	']': 2,
	'}': 3,
	'>': 4,
};

const PAIR_MAPPING: Record<string, string> = {
	'(': ')',
	'[': ']',
	'{': '}',
	'<': '>',
};

function scoreAutoComplete(symbolStack: string[]) {
	let result = 0;
	let symbol;
	while ((symbol = symbolStack.pop())) {
		result *= 5;
		result += AUTOCOMPLETE_SCORE[symbol];
	}
	return result;
}

/**
 * Check the line for syntax errors
 *
 * Returns the score of the syntax error, or `0` if the line is syntactically
 * fine, or the negative score of the autocompletion when the line is incomplete.
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
		return -scoreAutoComplete(expectedNextClosing);
	}
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	let syntaxErrorScore = 0;
	let autoCompleteScores: number[] = [];

	return new Promise((resolve, reject) => {
		rl.on('line', (line) => {
			// Process `line` here
			const s = syntaxCheck(line);
			if (s === 0) {
				// Complete, we don't care.
				return;
			}
			if (s < 0) {
				// Incomplete
				autoCompleteScores.push(-s);
			} else {
				// Error
				syntaxErrorScore += s;
			}
		});
		rl.on('error', (err) => {
			reject(err);
		});
		rl.on('close', () => {
			autoCompleteScores.sort((a, b) => a - b);
			console.log(
				`Results for ${input}: syntax error score ${syntaxErrorScore}, auto complete score ${
					autoCompleteScores[
						Math.floor(autoCompleteScores.length / 2)
					]
				}`
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
