#!/usr/bin/env node

import { createReadStream, fstat, writeFileSync } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	return new Promise((resolve, reject) => {
		const rows: number[][] = [];

		rl.on('line', (line) => {
			rows.push(line.split('').map((c) => parseInt(c, 10)));
		});
		rl.on('error', (err) => {
			reject(err);
		});
		rl.on('close', () => {
			const colorClasses = `
			.h4 { color: #ff4000ff; }
			.h3 { color: #fd7547ff; }
			.h2 { color: #fc906aff; }
			.h1 { color: #faaa8dff; }
			.h0 { color: #201e1fff; }
			`;

			// Walk over all elements, and check that that there is a smaller neighbor
			// somewhere. If there isn't, remember it as a low point.
			const debugMap: string[] = [];
			const lowestPointHeights: number[] = [];
			for (let rowIndex = 0; rowIndex < rows.length; rowIndex++) {
				let debugLine = '';
				const row = rows[rowIndex];
				for (let colIndex = 0; colIndex < row.length; colIndex++) {
					const height = row[colIndex];
					// XXX: The task isn't exactly clear, but it seems we're only a low point
					//      if none of our neighbors is at the same height or higher (i.e. plateaus don't count?)
					function isLower(other: number) {
						return other <= height;
					}

					let lowerNeighbors = 0;
					if (colIndex > 0 && isLower(row[colIndex - 1])) {
						lowerNeighbors++;
					}
					if (
						colIndex < row.length - 1 &&
						isLower(row[colIndex + 1])
					) {
						lowerNeighbors++;
					}
					if (rowIndex > 0 && isLower(rows[rowIndex - 1][colIndex])) {
						lowerNeighbors++;
					}
					if (
						rowIndex < rows.length - 1 &&
						isLower(rows[rowIndex + 1][colIndex])
					) {
						lowerNeighbors++;
					}
					const hasLowerNeighbor = lowerNeighbors > 0;
					let text;
					if (!hasLowerNeighbor) {
						lowestPointHeights.push(height);
						text = `<strong>${height}</strong>`;
					} else {
						text = `${height}`;
					}
					debugLine += `<span class="h${lowerNeighbors}">${text}</span>`;
				}
				debugMap.push(debugLine);
			}
			const totalRiskLevel = lowestPointHeights
				.map((height) => height + 1)
				.reduce((s, riskLevel) => s + riskLevel, 0);
			console.log(`Results for ${input}: ${totalRiskLevel}`);

			writeFileSync(
				'day-9-debug.html',
				`<!DOCTYPE html><html><head><style>${colorClasses}</style></head><body><pre>${debugMap.join(
					'\n'
				)}</pre></body></html>`,
				'utf-8'
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
