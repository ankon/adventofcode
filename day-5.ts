#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

const rl = createInterface(createReadStream(`${basename(process.argv[1], extname(process.argv[1]))}.input`));

const seats = Array(1024);

rl.on('line', line => {
	const row = parseInt(line.slice(0, 7).replace(/F/g, '0').replace(/B/g, '1'), 2);
	const col = parseInt(line.slice(7, 10).replace(/L/g, '0').replace(/R/g, '1'), 2);
	const id = row * 8 + col;
	seats[id] = 1
});
rl.on('close', () => {
	for (let i = 1; i < seats.length; i++) {
		if (!seats[i] && seats[i - 1] && seats[i + 1]) {
			console.log(`Seat with id ${i} is free`);
		}
	}
});