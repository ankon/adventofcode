#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

import { between } from '../utils';

type Rule = { min: number, max: number };
type Field = { name: string, rules: Rule[] };
type State = 'Rules'|'MyTicket'|'NearbyTickets';
type Ticket = number[];

const FIELD_RE = /^([a-z ]+): (.*)$/;
const RULE_RE = /(\d+)-(\d+)( or)?/g;

function applies(rule: Rule, value: number): boolean {
	return between(value, rule.min, rule.max);
}

function matchingFields(fields: Field[], value: number): Field[] {
	const result = [];
	for (const field of fields) {
		for (const rule of field.rules) {
			if (applies(rule, value)) {
				result.push(field);
				break;
			}
		}
	}
	return result;
}

function findInvalidValues(fields: Field[], ticket: Ticket): number[] {
	let result: number[] = [];
	for (const value of ticket) {
		if (matchingFields(fields, value).length === 0) {
			result.push(value);
		}
	}

	return result;
}

function assignFieldOrder(fields: Field[], validTickets: Ticket[]): Field[] {
	// Start out with all fields possible for each position
	const possibleFields: Field[][] = fields.map(field => ([...fields]));

	// Process the tickets, and restrict the possible fields
	for (const ticket of validTickets) {
		for (let i = 0; i < ticket.length; i++) {
			possibleFields[i] = matchingFields(possibleFields[i], ticket[i]);
		}
	}

	// We now should have (at least) one of the possible fields set to a single value.
	// Find that, remove the field from the other possible fields, and repeat.
	// XXX: We may still have to recurse/backtrack here, if there are multiple combinations. But
	//      this will be cheaper already because we know what is at most possible.
	let pending = fields.length;
	const result: Field[] = new Array(fields.length);

	while (pending > 0) {
		const knownFieldIndex = possibleFields.findIndex(possible => possible.length === 1);
		if (knownFieldIndex === -1) {
			throw new Error(`No next obvious field, still ${pending} to find`);
		}
		// Assign
		const knownField = possibleFields[knownFieldIndex][0];
		result[knownFieldIndex] = knownField;

		// Remove everywhere
		for (let i = 0; i < possibleFields.length; i++) {
			possibleFields[i] = possibleFields[i].filter(({ name }) => name !== knownField.name);
		}

		// Next!
		pending--;
	}
	return result;
}

function processInput(input: string) {
	const rl = createInterface(createReadStream(input));

	let state: State = 'Rules';

	const fields: Field[] = [];
	let myTicket: Ticket;
	const nearbyTickets: Ticket[] = [];

	rl.on('line', line => {
		if (line === 'your ticket:') {
			state = 'MyTicket';
		} else if (line === 'nearby tickets:') {
			state = 'NearbyTickets';
		} else if (state === 'Rules' && line !== '') {
			const [, name, rulesText] = FIELD_RE.exec(line)!;

			const rules: Rule[] = [];
			let ruleMatch = null;
			while ((ruleMatch = RULE_RE.exec(rulesText)) !== null) {
				rules.push({ min: Number(ruleMatch[1]), max: Number(ruleMatch[2]) });
			}
			fields.push({ name, rules });
		} else if (state === 'MyTicket' && line !== '') {
			myTicket = line.split(/,/).map(v => parseInt(v));
		} else if (state === 'NearbyTickets') {
			const ticket = line.split(/,/).map(v => parseInt(v));
			nearbyTickets.push(ticket);
		}
	});
	rl.on('close', () => {
		console.log(`Results for ${input}:`);
		let errorRate = 0;
		const validTickets: Ticket[] = [];
		for (const nearbyTicket of nearbyTickets) {
			const invalidValues = findInvalidValues(fields, nearbyTicket);
			if (invalidValues.length > 0) {
				errorRate += invalidValues.reduce((s, v) => s + v, 0);
			} else {
				validTickets.push(nearbyTicket);
			}
		}
		console.log(`Error rate is ${errorRate}`);

		const orderedFields: Field[] = assignFieldOrder(fields, validTickets);
		const checksum = myTicket.reduce((r, v, i) => {
			const field = orderedFields[i];
			if (field.name.startsWith('departure')) {
				return r * v;
			} else {
				return r;
			}
		}, 1);
		console.log(`Checksum is ${checksum}`);
	});
}

processInput(`${basename(process.argv[1], extname(process.argv[1]))}-example.input`);
processInput(`${basename(process.argv[1], extname(process.argv[1]))}.input`);
