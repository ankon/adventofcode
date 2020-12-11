export function qs(numbers: number[], number: number): number {
	let left = 0;
	let right = numbers.length;
	while (right - left > 0) {
		const mid = left + Math.floor((right - left) / 2);
		if (number < numbers[mid]) {
			right = mid;
		} else if (number > numbers[mid]) {
			left = mid + 1;
		} else {
			return mid;
		}
	}
	return -left - 1;
}
