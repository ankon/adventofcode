#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

const START = 'start';
const END = 'end';

type DiGraph = Record<string, string[]>;
type Path = string;

function canGoIntoPart1(visitedCaves: string[], cave: string): boolean {
	return cave.toLowerCase() === cave ? !visitedCaves.includes(cave) : true;
}

function canGoIntoPart2(visitedCaves: string[], cave: string): boolean {
	if (cave === START) {
		// No need to check, we came from there
		return false;
	} else if (cave === END) {
		// Fine to go into, and we know the caller won't ask again
		return true;
	} else if (cave.toUpperCase() === cave) {
		return true;
	}

	// small cave, check whether the invariant requested still holds: At most
	// one small cave visited twice.
	const counters: Record<string, number> = { [cave]: 1 };
	let seenDoubleVisit = false;
	for (const visitedCave of visitedCaves) {
		if (visitedCave.toUpperCase() === visitedCave) {
			continue;
		}
		const newCount = (counters[visitedCave] ?? 0) + 1;
		counters[visitedCave] = newCount;

		// Can be 3 if we have a double visit for the requested cave already
		if (newCount >= 2) {
			if (seenDoubleVisit) {
				return false;
			}
			seenDoubleVisit = true;
		}
	}
	return true;
}

function countPaths(
	graph: DiGraph,
	openPaths: Set<Path>,
	canGoInto = canGoIntoPart1
): number {
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
			const pathsP2 = countPaths(graph, new Set([START]), canGoIntoPart2);
			console.log(
				`Results for ${input}: ${paths} paths (${pathsP2} for variant 2)`
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
	'-example-1',
	'-example-2',
	'-example-3',
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
