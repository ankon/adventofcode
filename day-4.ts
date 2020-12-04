#!/usr/bin/env node

import { createInterface } from 'readline';
import { createReadStream } from 'fs';

function between(v: number, min: number, max: number) {
	return v >= min && v <= max;
}

interface Requirement {
	field: string;
	isValid: (value: string) => boolean;
}

const REQUIREMENTS_PART_1: Requirement[] = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']
	.map(field => ({ field, isValid: value => Boolean(value)}));

const REQUIREMENTS_PART_2: Requirement[] = [
	{ field: 'byr', isValid: value => between(Number(value), 1920, 2002) },
	{ field: 'iyr', isValid: value => between(Number(value), 2010, 2020) },
	{ field: 'eyr', isValid: value => between(Number(value), 2020, 2030) },
	{
		field: 'hgt',
		isValid: value => {
			const match = /^([0-9]+)(cm|in)$/.exec(value);
			if (match) {
				const [, v, unit] = match;
				switch (unit) {
					case 'cm': return between(Number(v), 150, 193);
					case 'in': return between(Number(v), 59, 76);
				}
			}
			return false;
		},
	},
	{ field: 'hcl', isValid: value => /^#[0-9a-f]{6}$/.test(value) },
	{ field: 'ecl', isValid: value => ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'].includes(value) },
	{ field: 'pid', isValid: value => /^[0-9]{9}$/.test(value) },
];

let passport: Record<string, string> = {};

let validPart1 = 0;
let validPart2 = 0;

const rl = createInterface(createReadStream('day-4.input'));
rl.on('line', line => {
	if (line.trim() === '') {
		// Process current passport
		if (REQUIREMENTS_PART_1.every(({field, isValid}) => isValid(passport[field]))) {
			validPart1++;
		}
		const invalidFieldsPart2 = REQUIREMENTS_PART_2.filter(({field, isValid}) => !isValid(passport[field])).map(({ field }) => field);
		if (invalidFieldsPart2.length === 0) {
			validPart2++;
		} else {
			console.log(`Invalid fields ${JSON.stringify(invalidFieldsPart2)}: ${JSON.stringify(passport)}`);
		}
		// Reset passport
		passport = {};
	} else {
		for (const kv of line.split(/\s+/)) {
			const [field, value] = kv.split(/:/);
			passport[field] = value;
		}
	}
});
rl.on('close', () => {
	console.log(`Found ${validPart1} part-1-valid passports`);
	console.log(`Found ${validPart2} part-2-valid passports`);
});