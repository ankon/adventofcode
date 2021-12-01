#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

type Instruction = 'nop'|'acc'|'jmp';
interface Statement {
	instruction: Instruction;
	argument: number;
}
type Program = Statement[];

const PATCH_TABLE = {
	'nop': 'jmp' as const,
	'acc': 'acc' as const,
	'jmp': 'nop' as const,
};

function runProgram(program: Program, patchAt?: number): { acc: number, aborted: boolean, terminated: boolean } {
	const executedStatementIndices: number[] = [];

	let acc = 0;
	let ip = 0;
	let abort = false;

	while (!executedStatementIndices.includes(ip) && ip !== program.length) {
		if (ip < 0 || ip > program.length) {
			abort = true;
			break;
		}

		executedStatementIndices.push(ip);

		let { instruction, argument } = program[ip];
		if (ip === patchAt) {
			instruction = PATCH_TABLE[instruction];
		}

		switch (instruction) {
			case 'nop':
				ip++;
				break;
			case 'acc':
				acc += argument;
				ip++;
				break;
			case 'jmp':
				ip += argument;
				break;
		}
	}

	return { acc, aborted: abort, terminated: ip === program.length };
}

function processInput(input: string) {
	const rl = createInterface(createReadStream(input));

	const program: Statement[] = [];
	rl.on('line', line => {
		const tmp = line.split(/\s/);
		program.push({
			instruction: tmp[0] as Instruction,
			argument: Number(tmp[1]),
		});
	});
	rl.on('close', () => {
		const { acc: result } = runProgram(program);
		console.log(`Results for ${input}:`);
		console.log(`Unmodified execution: ${result}`);

		for (let patchAt = 0; patchAt < program.length; patchAt++) {
			const { acc, aborted, terminated } = runProgram(program, patchAt);
			if (terminated) {
				console.log(`Patch at ${patchAt} fixes the program: ${acc}`);
				break;
			} else if (aborted) {
				console.log(`Patch at ${patchAt} leads to program abort`);
			}
		}
	});
}

processInput(`${basename(process.argv[1], extname(process.argv[1]))}-example.input`);
processInput(`${basename(process.argv[1], extname(process.argv[1]))}.input`);
