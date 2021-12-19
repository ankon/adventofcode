#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

class Bitstream {
	private consumedLength = 0;

	constructor(
		private data: Buffer,
		/** Offset (in bits) */
		private offset = 0,
		/** Maximum number of bits to consume */
		private length = data.length * 8
	) {}

	private consume(length: number) {
		if (this.consumedLength + length > this.length) {
			throw new Error(
				`Cannot consume ${length} bits (only ${this.available} available)`
			);
		}
		this.consumedLength += length;
	}

	public get available() {
		return this.length - this.consumedLength;
	}

	public readBitstream(length: number): Bitstream {
		this.consume(length);

		const result = new Bitstream(this.data, this.offset, length);
		this.offset += length;
		return result;
	}

	/** Read the next `num` bits and return the value */
	public readBits(num: number): number {
		this.consume(num);

		let result = 0;
		// TODO: We could special-case the situation where we need more than a byte
		//      by first proceeding to the full next byte, then grab bytes until we need
		//      an incomplete byte, and then grab the remaining bits.
		while (num > 0) {
			const byteOffset = Math.floor(this.offset / 8);
			// Grab the byte we need, mask off the needed bits and put them into our number
			const byte = this.data[byteOffset];

			/** Number of bits to shift the result left */
			let consumedBits = 0;
			/** Mask for picking the right bits from byte */
			let mask = 0;

			/** First bit in this byte we're looking at */
			let firstBit = this.offset % 8;
			for (let bit = firstBit; bit < 8; bit++) {
				mask <<= 1;
				if (num > 0) {
					mask |= 1;
					consumedBits++;
					num--;
				}
			}
			this.offset += consumedBits;

			const rawBits = byte & mask;
			const rawBitsShift = 8 - firstBit - consumedBits;
			result = (result << consumedBits) | (rawBits >> rawBitsShift);
		}
		return result;
	}

	public readVersion() {
		return this.readBits(3);
	}

	public readType() {
		return this.readBits(3);
	}

	/** Read a literal value */
	public readLiteralValue() {
		const HAS_NEXT_BIT = 1 << 4;
		let result = 0;
		let hasNext;
		do {
			const bits = this.readBits(5);
			hasNext = (bits & HAS_NEXT_BIT) === HAS_NEXT_BIT;
			result = (result << 4) | (bits & 0xf);
		} while (hasNext);
		return result;
	}
}

/** Known packet type for "literal value" */
const PT_LITERAL = 4;

interface Packet {
	version: number;
	type: number;
}

interface LiteralValue extends Packet {
	value: number;
}

interface Operator extends Packet {
	operands: Packet[];
}

function parse(bitstream: Bitstream): Packet {
	const version = bitstream.readVersion();
	const type = bitstream.readType();
	if (type === PT_LITERAL) {
		// A literal (shouldn't happen as top-level)
		const value = bitstream.readLiteralValue();
		const result: LiteralValue = {
			version,
			type,
			value,
		};
		return result;
	} else {
		// Some operator, so the next part defines how to walk through it
		const operands: Packet[] = [];
		const lengthType = bitstream.readBits(1);
		if (lengthType === 0) {
			const totalLengthInBits = bitstream.readBits(15);
			const slice = bitstream.readBitstream(totalLengthInBits);
			while (slice.available) {
				operands.push(parse(slice));
			}
		} else {
			const numSubPackets = bitstream.readBits(11);
			for (let i = 0; i < numSubPackets; i++) {
				operands.push(parse(bitstream));
			}
		}

		const result: Operator = {
			version,
			type,
			operands,
		};
		return result;
	}
}

function isLiteralValue(packet: Packet): packet is LiteralValue {
	return packet.type === PT_LITERAL;
}

function isOperator(packet: Packet): packet is Operator {
	return packet.type !== PT_LITERAL;
}

function calculateVersionSum(packet: Packet): number {
	if (isLiteralValue(packet)) {
		return packet.version;
	} else if (isOperator(packet)) {
		return (
			packet.version +
			packet.operands.reduce((s, p) => s + calculateVersionSum(p), 0)
		);
	}
	throw new Error('Unexpected packet');
}

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	return new Promise((resolve, reject) => {
		let data: Buffer = Buffer.alloc(0);
		rl.on('line', (line) => {
			// Parse the line as hex-encoded data, and concat it
			// The concat isn't strictly needed, because it seems there is only one line
			// coming, but it doesn't hurt.
			data = Buffer.concat([data, Buffer.from(line, 'hex')]);
		});
		rl.on('error', (err) => {
			reject(err);
		});
		rl.on('close', () => {
			const packet = parse(new Bitstream(data));
			const versionSum = calculateVersionSum(packet);
			console.log(`Results for ${input}: ${versionSum}`);

			resolve();
		});
	});
}

async function main(inputFiles: string[]) {
	for (const inputFile of inputFiles) {
		try {
			await processInput(inputFile);
		} catch (err: any) {
			console.error(`Cannot process ${inputFile}: ${err.message}`);
		}
	}
}

const INPUT_SPECS = [
	//
	// Literal 2021
	// '-example-literal',
	// Op [10, 20]
	// '-example-operator-LT0',
	// Op [1, 2, 3]
	// '-example-operator-LT1',
	// Version sum 16
	'-example-1',
	// Version sum 12
	'-example-2',
	// Version sum 23
	'-example-3',
	// Version sum 31
	'-example-4',
	'',
];

main(
	INPUT_SPECS.map(
		(inputSpec) =>
			`${basename(
				process.argv[1],
				extname(process.argv[1])
			)}${inputSpec}.input`
	)
).catch((err) => console.error(err));
