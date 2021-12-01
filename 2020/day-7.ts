#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

interface Container {
	color: string;
	contains: {
		number: number;
		color: string;
	}[];
}

function part1(start: string, containers: Container[]) {
	let open: string[] = [start];

	const containedIn: string[] = [];

	// Walk over the rules, and find our search list
	// Then repeat, and find those
	while (open.length > 0) {
		const processing = open;
		open = [];
		for (const container of containers) {
			if (containedIn.includes(container.color)) {
				continue;
			}
			if (container.contains.some(({ color }) => processing.includes(color))) {
				open.push(container.color);
				containedIn.push(container.color);
			}
		}
		console.debug(`${processing} bags can be found inside ${open} bags`);
	}
	console.log(`${containedIn.length} options to contain one ${start} bag: ${containedIn}`);
}

function part2(start: string, containers: Container[]) {
	let open: { color: string, number: number }[] = [{ color: start, number: 1 }];

	const contents: {[color: string]: number} = {};

	while (open.length > 0) {
		const processing = open;
		open = [];
		for (const required of processing) {
			const container = containers.find(({ color }) => color === required.color)!;
			for (const contained of container.contains) {
				open.push({ color: contained.color, number: required.number * contained.number });
				contents[contained.color] = (contents[contained.color] || 0) + required.number * contained.number;
			}
		}
	}
	console.log(`${Object.values(contents).reduce((result, number) => result + number, 0)} bags contained in one ${start} bag: ${JSON.stringify(contents)}`);
}

function processInput(input: string) {
	const containers: Container[] = [];

	const RULE_RE = /^([a-z ]+) bags contain (no other bags|.*).$/;
	const CONTAINMENT_RE = /(\d+) ([a-z ]+) bags?(, )?/g;

	const rl = createInterface(createReadStream(input));

	rl.on('line', line => {
		const match = RULE_RE.exec(line);
		if (!match) {
			throw new Error(`Failed to parse rule "${line}"`);
		}
		const [, color, containment] = match;
		const container: Container = {
			color,
			contains: [],
		};
		if (containment !== 'no other bags') {
			let containmentMatch: RegExpExecArray|null;
			while ((containmentMatch = CONTAINMENT_RE.exec(containment)) !== null) {
				container.contains.push({
					number: Number(containmentMatch[1]),
					color: containmentMatch[2],
				});
			}
		}
		containers.push(container);
	});
	rl.on('close', () => {
		console.log(`Results for ${input}:`);

		// Part 1: How many options to contain a "shiny gold" bag?
		part1('shiny gold', containers);

		// Part 2: What would be inside the "shiny gold" bag (ignoring topology)
		part2('shiny gold', containers);
	});
}

processInput(`${basename(process.argv[1], extname(process.argv[1]))}-example.input`);
processInput(`${basename(process.argv[1], extname(process.argv[1]))}-example-2.input`);
processInput(`${basename(process.argv[1], extname(process.argv[1]))}.input`);