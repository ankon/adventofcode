#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

const rl = createInterface(createReadStream(`${basename(process.argv[1], extname(process.argv[1]))}.input`));

rl.on('line', line => {
});
rl.on('close', () => {
});