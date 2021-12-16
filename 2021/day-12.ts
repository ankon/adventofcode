#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

const START = 'start';
const END = 'end';

type DiGraph = Record<string, string[]>;
type Path = string;

function canGoInto(visitedCaves: string[], cave: string): boolean {
	return cave.toLowerCase() === cave ? !visitedCaves.includes(cave) : true;
}
function countPaths(graph: DiGraph, openPaths: Set<Path>): number {
	let result = 0;
	let closedPaths: Set<Path> = new Set();

	// While there are open paths: Assume the first one is active, and
	// expand it for all possible targets from its last node. If we reach
	// END: consider that one closed, increase the count, and pick the next one to continue
	// with.
	// Considerations:
	// - we need to avoid creating multiple same open paths
	// - we need to avoid creating a new open path that matches a previously closed one
	// - presumably a loop A-B doesn't exist, as we cannot exhaustively count the paths
	//   (IOW: it would be infinitely many, but we must not try that!)
	while (openPaths.size > 0) {
		// A stack-ish interface would be nice, or just an iterator.remove() ...
		const active = openPaths.values().next().value as string;
		openPaths.delete(active);

		const visitedCaves = active.split(/-/);
		const last = visitedCaves[visitedCaves.length - 1];
		for (let nextCave of graph[last]) {
			if (!canGoInto(visitedCaves, nextCave)) {
				continue;
			}
			const potential = `${active}-${nextCave}`;
			if (closedPaths.has(potential) || openPaths.has(potential)) {
				continue;
			}
			if (nextCave === END) {
				closedPaths.add(potential);
				result++;
			} else {
				openPaths.add(potential);
			}
		}
	}
	return result;
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	const graph: DiGraph = {};

	return new Promise((resolve, reject) => {
		rl.on('line', (line) => {
			const [from, to] = line.split(/-/);

			// Throw into our DiGraph structure
			let targets = graph[from] ?? [];
			targets.push(to);
			graph[from] = targets;
			targets = graph[to] ?? [];
			targets.push(from);
			graph[to] = targets;
		});
		rl.on('error', (err) => {
			reject(err);
		});
		rl.on('close', () => {
			const paths = countPaths(graph, new Set([START]));
			console.log(`Results for ${input}: ${paths} paths`);
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
	// '-example-1',
	// '-example-2',
	// '-example-3',
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
