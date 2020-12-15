#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

function maskToString(mask: bigint): string {
	return mask.toString(2).padStart(36, '0');
}

type Address = bigint;
type Value = bigint;

const BANK_SIZE = 1048576;

abstract class Decoder {
	// Banked memory
	private readonly memoryBanks: Value[][] = new Array();

	constructor(
		public readonly name: string,
		protected readonly bankSize: number = BANK_SIZE,
		protected readonly debugEnabled: boolean = false,
	) {}

	public get sum() {
		let lastProgress = 0;
		const banks = this.memoryBanks.length;
		return this.memoryBanks.reduce((r, bank, i) => {
			if (this.debugEnabled) {
				process.stdout.write(`\r...${i}/${banks} banks done: ${bank.length} bytes in this one`);
				lastProgress = i;
			} else if (lastProgress && i === banks - 1) {
				process.stdout.write(`\r`);
			}
			return bank.reduce((bankSum, v) => bankSum + v, r);
		}, BigInt(0));
	}

	public abstract opMask(mask: string): void;
	public abstract opMem(address: Address, value: Value): void;

	protected memSet(address: Address, value: Value): void {
		const bankIndex = Number(address / BigInt(this.bankSize));
		let bank = this.memoryBanks[bankIndex];
		if (!bank) {
			bank = this.memoryBanks[bankIndex] = Array();
		}

		const bankAddress = Number(address % BigInt(this.bankSize));
		bank[bankAddress] = value;
	}

	protected debug(...args: any[]) {
		if (this.debugEnabled) {
			console.debug(...args);
		}
	}
}

export class DecoderV1 extends Decoder {
	private orMask = BigInt(0);
	private andMask = BigInt(parseInt('0xFFFFFFFFF'));

	constructor(bankSize?: number, debugEnabled?: boolean) {
		super('V1', bankSize, debugEnabled);
	}

	public opMask(mask: string) {
		// Mask is an odd trinary thing, with '0'/'1' in a position setting the value at that bit position, and
		// 'X' in a position to mean "unchanged"
		// One could treat this single mask as two separate ones: Interpreting 'X'/'1' as 1 and '0' as 0 produces
		// "AndMask", and 'X'/'0' as 0 and '1' as 1 produces the "OrMask".
		// The final value then is (value & AndMask) | OrMask
		this.andMask = BigInt(parseInt(mask.replace(/[X1]/g, '1'), 2));
		this.orMask = BigInt(parseInt(mask.replace(/[X0]/g, '0'), 2));
		this.debug(`mask = [AND = ${maskToString(this.andMask)}, OR = ${maskToString(this.orMask)}] {${mask}}`);
	}

	public opMem(address: Address, value: Value) {
		const maskedValue = this.applyMask(value);
		this.memSet(address, maskedValue);
		this.debug(`mem[${address}] = [${maskedValue}] {${value}}`);
	}

	private applyMask(value: Value): Value {
		return (value & this.andMask) | this.orMask;
	}
}

export class DecoderV2 extends Decoder {
	private orMask = BigInt(0);
	private floatMask = BigInt(0);
	private floatingMasks = [BigInt(0)];

	constructor(bankSize?: number, debugEnabled?: boolean) {
		super('V2', bankSize, debugEnabled);
	}

	public opMask(mask: string) {
		// Process the value:
		let newOrMask = BigInt(0);
		// For each 'X' we add two masks: One where the 'X' is a 0, and one where it is a 1.
		let newFloatingMasks = [BigInt(0)];
		let newFloatMask = BigInt(0);

		function invariants(allowZero: boolean, check: boolean) {
			if (check) {
				// Sanity check: We should have 2^popcnt(floatMask) unique floating masks, and at least 1
				const expectedFloatingMasks = 2 ** (newFloatMask.toString(2).match(/1/g)?.length || 0);
				if (expectedFloatingMasks) {
					if (expectedFloatingMasks !== newFloatingMasks.length) {
						throw new Error(`unexpected number of floating masks`);
					}
					if (new Set(newFloatingMasks).size !== expectedFloatingMasks) {
						throw new Error(`non-unique floating masks`);
					}
				} else if (!allowZero) {
					throw new Error('must have at least one floating bit');
				}
			}
		}

		for (let i = 0; i < mask.length; i++) {
			const c = mask[i];
			const bit = BigInt(1) << BigInt(36 - i - 1);
			if (c === '1') {
				// Set the bit in all existing masks
				newFloatingMasks = newFloatingMasks.map(newMask => newMask | bit);
				newOrMask = newOrMask | bit;
			} else if (c === 'X') {
				// Copy the masks and set the bit to one in the copies
				newFloatingMasks = [
					// Masks where the bit stays 0
					...newFloatingMasks,
					// Masks where the bit is 1
					...newFloatingMasks.map(newMask => newMask | bit),
				];

				// Remember that this was a floating bit
				newFloatMask = newFloatMask | bit;
			} else {
				// Nothing to do, bit was initialized to 0
			}

			invariants(true, this.debugEnabled);
		}

		invariants(false, this.debugEnabled);

		this.orMask = newOrMask;
		this.floatingMasks = newFloatingMasks;
		this.floatMask = newFloatMask;
		this.debug(`mask = [OR = ${maskToString(this.orMask)}, FLOAT = ${maskToString(this.floatMask)}, ${this.floatingMasks.length} floating] {${mask}}`);
	}

	public opMem(address: Address, value: Value) {
		const maskedAddresses = this.maskedAddresses(address);
		for (const maskedAddress of maskedAddresses) {
			this.memSet(maskedAddress, value);
		}
		this.debug(`mem[${maskedAddresses.length} addresses] = ${value}`);
	}

	private maskedAddresses(address: Address): Address[] {
		// Process the address:
		// 1. Apply the orMask to all non-floating bits
		const orMasked = BigInt(address) & ~this.floatMask | this.orMask;
		// 2. Apply each of the floating masks to the floating bits
		return this.floatingMasks.map(mask => orMasked | (mask & this.floatMask));
	}
}

const STMT_RE = /^((mask) = ([X01]+)|(mem)\[(\d+)\] = (\d+))$/;

function processInput(input: string, ...decoders: Decoder[]) {
	const rl = createInterface(createReadStream(input));

	rl.on('line', line => {
		const match = STMT_RE.exec(line);
		if (!match) {
			throw new Error(`Invalid syntax: ${line}`);
		}

		if (match[2] === 'mask') {
			const maskValue = match[3];
			decoders.forEach(decoder => decoder.opMask(maskValue));
		} else if (match[4] === 'mem') {
			const address = BigInt(match[5]);
			const sourceValue = BigInt(match[6]);
			decoders.forEach(decoder => decoder.opMem(address, sourceValue));
		} else {
			throw new Error('ASSERTION: Cannot happen');
		}
	});
	rl.on('close', () => {
		decoders.forEach(decoder => {
			console.log(`Results for ${input} and decoder ${decoder.name}: ${decoder.sum}`);
		});
	});
}

processInput(`${basename(process.argv[1], extname(process.argv[1]))}-example.input`, new DecoderV1());
// The initial example is "horrible" when using the v2 decoder, due to the amount of floating bits in the mask.
//processInput(`${basename(process.argv[1], extname(process.argv[1]))}-example.input`, new DecoderV2());
processInput(`${basename(process.argv[1], extname(process.argv[1]))}-example-2.input`, new DecoderV2());
processInput(`${basename(process.argv[1], extname(process.argv[1]))}.input`, new DecoderV1(), new DecoderV2());
