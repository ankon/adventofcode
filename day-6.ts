#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

const rl = createInterface(createReadStream(`${basename(process.argv[1], extname(process.argv[1]))}.input`));

const ALL = Array.from('abcdefghijklmnopqrstuvwxyz');

let resultPart1 = 0;
let resultPart2 = 0;
let currentPart1 = new Set();
let currentPart2 = new Set(ALL);

rl.on('line', line => {
    if (line.trim()) {
        // More answers for the current group
        Array.from(line).forEach(answer => currentPart1.add(answer));
        for (const answer of currentPart2.values()) {
            if (line.indexOf(answer) === -1) {
                currentPart2.delete(answer);
            }
        }
    } else {
        // Group change, count and add
        resultPart1 += currentPart1.size;
        currentPart1.clear();
        resultPart2 += currentPart2.size;
        ALL.forEach(answer => currentPart2.add(answer));
    }
});
rl.on('close', () => {
    resultPart1 += currentPart1.size;
    resultPart2 += currentPart2.size;

    console.log(`Got in total part1 ${resultPart1} part2 ${resultPart2}`);
});