#!/usr/bin/env node

import { createInterface } from 'readline';
import { createReadStream } from 'fs';

const dxs = [1, 3, 5, 7];

const counts: {[dx: number]: number} = dxs.reduce((r, v) => ({...r, [v]: 0}), {});
let countDY2 = 0;

let xDY2 = 0;
let y = 0;
const xs: {[dx: number]: number} = dxs.reduce((r, v) => ({...r, [v]: 0}), {});

const rl = createInterface(createReadStream('day-3.input'));
rl.on('line', line => {
	for (const dx of dxs) {
		counts[dx] += line[xs[dx] % line.length] === '#' ? 1 : 0; 
		xs[dx] += dx;
	}
	if (y % 2 === 0) {
		countDY2 += line[xDY2 % line.length] === '#' ? 1 : 0; 
		xDY2++;
	}
	y++;
});
rl.on('close', () => {
	console.log(`Saw ${counts[3]} trees`);

	console.log(`Part2: ${counts[1] * counts[3] * counts[5] * counts[7] * countDY2}`);
});