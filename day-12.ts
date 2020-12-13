#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

type Command = 'N'|'E'|'S'|'W'|'F'|'R'|'L';

interface Point {
	x: number;
	y: number;
}

interface State {
	ship: Point;
	waypoint: Point;
}

function rotate({ x, y }: Point, turnsRight: /* -4<=turnsRight<=+Inf */ number): Point {
	let nx = x;
	let ny = y;
	for (let i = 0; i < (turnsRight + 4) % 4; i++) {
		const tmp = nx;
		nx = ny;
		ny = -tmp;
	}
	return {
		x: nx,
		y: ny,
	};
}

function updateStatePart1(state: State, command: Command, argument: number): State {
	let { ship, waypoint } = state;

	switch (command) {
	case 'N':
		ship.y += argument;
		break;
	case 'E':
		ship.x += argument;
		break;
	case 'S':
		ship.y -= argument;
		break;
	case 'W':
		ship.x -= argument;
		break;
	case 'F':
		ship.x += waypoint.x * argument;
		ship.y += waypoint.y * argument;
		break;
	case 'L':
		waypoint = rotate(waypoint, -argument / 90);
		break;
	case 'R':
		waypoint = rotate(waypoint, argument / 90);
		break;
	}

	return { ship, waypoint };
}

function updateStatePart2(state: State, command: Command, argument: number): State {
	let { ship, waypoint } = state;

	switch (command) {
	case 'N':
		waypoint.y += argument;
		break;
	case 'E':
		waypoint.x += argument;
		break;
	case 'S':
		waypoint.y -= argument;
		break;
	case 'W':
		waypoint.x -= argument;
		break;
	case 'F':
		ship.x += waypoint.x * argument;
		ship.y += waypoint.y * argument;
		break;
	case 'L':
		waypoint = rotate(waypoint, -argument / 90);
		break;
	case 'R':
		waypoint = rotate(waypoint, argument / 90);
		break;
	}

	return { ship, waypoint };
}

const CASES = [
	{ initialState: { ship: { x: 0, y: 0 }, waypoint: { x: 1, y: 0 } }, updateState: updateStatePart1 },
	{ initialState: { ship: { x: 0, y: 0 }, waypoint: { x: 10, y: 1 } }, updateState: updateStatePart2 },
];

function processInput({ initialState, updateState }: typeof CASES[0], input: string) {
	const rl = createInterface(createReadStream(input));

	// Deep-clone the initial state
	let state: State = JSON.parse(JSON.stringify(initialState));

	rl.on('line', line => {
		let command = line[0] as Command;
		const argument = parseInt(line.substring(1));

		state = updateState(state, command, argument);
		console.log(`${input} ${updateState.name}: After ${line}: ${JSON.stringify(state.ship)} facing towards ${JSON.stringify(state.waypoint)}`);
	});

	rl.on('close', () => {
		console.log(`${input} ${updateState.name}: Manhattan distance ${Math.abs(state.ship.x) + Math.abs(state.ship.y)}`);
	});
}

processInput(CASES[0], `${basename(process.argv[1], extname(process.argv[1]))}-example.input`);
processInput(CASES[1], `${basename(process.argv[1], extname(process.argv[1]))}-example.input`);
processInput(CASES[1], `${basename(process.argv[1], extname(process.argv[1]))}.input`);
