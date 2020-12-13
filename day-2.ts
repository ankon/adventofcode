#!/usr/bin/env node

import { createInterface } from 'readline';
import { createReadStream } from 'fs';

import { between } from './utils';

const POLICY_RE = /^(\d+)-(\d+) (.)/;


function parseOriginalPolicy(policy: string): (password: string) => boolean {
	const [, min, max, policyCharacter] = POLICY_RE.exec(policy)!;

	return password => {
		const count = Array.from(password).reduce((result, c) => c === policyCharacter ? result + 1 : result, 0);
		return between(count, Number(min), Number(max));
	};
}

function parseRevisedPolicy(policy: string): (password: string) => boolean {
	const [, pos1, pos2, policyCharacter] = POLICY_RE.exec(policy)!;

	return password => {
		return (password.length >= Number(pos1) && password[Number(pos1) - 1] === policyCharacter) !== (password.length >= Number(pos2) && password[Number(pos2) - 1] === policyCharacter);
	}
}

let validOriginal = 0;
let validRevised = 0;

const INPUT_RE = /^([^:]+): (.+)$/
const rl = createInterface(createReadStream('day-2.input'));
rl.on('line', line => {
	const [, policy, password] = INPUT_RE.exec(line)!;
	const validateOriginal = parseOriginalPolicy(policy);
	if (validateOriginal(password)) {
		validOriginal++;
	}
	const validateRevised = parseRevisedPolicy(policy);
	if (validateRevised(password)) {
		validRevised++;
	}
});
rl.on('close', () => {
	console.log(`Found ${validOriginal} valid passwords according to original policy`);
	console.log(`Found ${validRevised} valid passwords according to revised policy`);
});