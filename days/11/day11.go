package _9

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"

	"github.com/ankon/adventofcode/2022/days"
	"github.com/ankon/adventofcode/2022/utils"
	"github.com/spf13/cobra"
	"golang.org/x/exp/slices"
)

//go:embed sample.txt
var sampleInput string

//go:embed input.txt
var fullInput string

var debug = 0
var rounds = 20
var relief = true

func ConfigureCommand(cmd *cobra.Command) {
	cmd.Flags().IntVar(&debug, "debug", debug, "Enable debug output (higher numbers mean more output)")
	cmd.Flags().IntVar(&rounds, "rounds", rounds, "Number of rounds (20 for task 1, 10000 for task 2)")
	cmd.Flags().BoolVar(&relief, "relief", relief, "Feel relief after inspection (true for task 1, false for task 2)")
}

func Run(useSampleInput bool) error {
	input := days.PickInput(useSampleInput, sampleInput, fullInput)
	lines := strings.Split(strings.TrimSpace(input), "\n")

	monkeys, err := parseMonkeys(lines)
	if err != nil {
		return err
	}

	activities := countItemInspections(monkeys, rounds)
	slices.Sort(activities)
	mostActivities := activities[len(activities)-1]
	secondMostActivities := activities[len(activities)-2]

	fmt.Printf("inspections: %d * %d = %d\n", mostActivities, secondMostActivities, mostActivities*secondMostActivities)

	return nil
}

type opFunc func(int) int

type monkey struct {
	// Items the monkey has (only tracking the worry level)
	items []int

	op opFunc

	divisor         int
	testTrueMonkey  int
	testFalseMonkey int

	inspections int
}

func (m *monkey) inspectItem() (int, bool) {
	if len(m.items) == 0 {
		return 0, false
	}
	result := m.items[0]
	m.items = m.items[1:]
	m.inspections++
	return result, true
}

func (m *monkey) catchItem(level int) {
	m.items = append(m.items, level)
}

const startItemsPrefix = "  Starting items: "
const opPrefix = "  Operation: new = "
const testDivisibleByPrefix = "  Test: divisible by "
const testIfTrueThrowPrefix = "    If true: throw to monkey "
const testIfFalseThrowPrefix = "    If false: throw to monkey "

var modulo = 1

func parseMonkey(lines []string) (monkey, int, error) {
	items := []int{}
	var op opFunc
	var divisor int
	var testTrueMonkey int
	var testFalseMonkey int

	i := 0
	for ; i < len(lines); i++ {
		line := lines[i]
		if line == "" {
			break
		} else if strings.HasPrefix(line, startItemsPrefix) {
			ss := strings.Split(line[len(startItemsPrefix):], ",")
			for _, s := range ss {
				v, err := strconv.Atoi(strings.TrimSpace(s))
				if err != nil {
					return monkey{}, i, fmt.Errorf("cannot parse starting items %q", ss)
				}
				items = append(items, v)
			}
		} else if strings.HasPrefix(line, opPrefix) {
			ss := strings.Split(line[len(opPrefix):], " ")
			arg2, _ := strconv.Atoi(ss[2])

			switch ss[1] {
			case "+":
				if ss[0] != "old" {
					return monkey{}, i, fmt.Errorf("first operand must be %q for +", "old")
				}
				if ss[2] == "old" {
					return monkey{}, i, fmt.Errorf("second operand must not be %q for +", ss[2])
				}
				op = func(item int) int {
					return (item + arg2) % modulo
				}
			case "*":
				if ss[0] != "old" {
					return monkey{}, i, fmt.Errorf("first operand must be %q for *", "old")
				}
				if ss[2] != "old" && !utils.IsPrime(arg2) {
					return monkey{}, i, fmt.Errorf("second operand must be %q or prime for *", "old")
				}
				op = func(item int) int {
					if ss[2] == "old" {
						return int(utils.MultiplyMod(uint64(item), uint64(item), uint64(modulo)))
					} else {
						return int(utils.MultiplyMod(uint64(item), uint64(arg2), uint64(modulo)))
					}
				}
			default:
				return monkey{}, i, fmt.Errorf("unsupported operator %q", ss[1])
			}
		} else if strings.HasPrefix(line, testDivisibleByPrefix) {
			v, err := strconv.Atoi(line[len(testDivisibleByPrefix):])
			if err != nil {
				return monkey{}, i, fmt.Errorf("cannot parse test %q", line)
			}
			divisor = v

			ifTrueLine := lines[i+1]
			v, err = strconv.Atoi(ifTrueLine[len(testIfTrueThrowPrefix):])
			if err != nil {
				return monkey{}, i, fmt.Errorf("cannot parse expected true condition %q", ifTrueLine)
			}
			testTrueMonkey = v

			ifFalseLine := lines[i+2]
			v, err = strconv.Atoi(ifFalseLine[len(testIfFalseThrowPrefix):])
			if err != nil {
				return monkey{}, i, fmt.Errorf("cannot parse expected false condition %q", ifFalseLine)
			}
			testFalseMonkey = v

			i += 2
		}
	}

	return monkey{items, op, divisor, testTrueMonkey, testFalseMonkey, 0}, i, nil
}

func parseMonkeys(lines []string) ([]*monkey, error) {
	result := []*monkey{}
	for l := 0; l < len(lines); l++ {
		line := lines[l]
		if strings.TrimSpace(line) == "" {
			continue
		}

		id := 0
		n, err := fmt.Sscanf(line, "Monkey %d:", &id)
		if err != nil {
			return nil, fmt.Errorf("cannot parse monkey line at %d", l)
		}
		if n != 1 {
			return nil, fmt.Errorf("expected monkey line at %d", l)
		}
		if id != len(result) {
			return nil, fmt.Errorf("expected monkey id %d, but got %d at %d", len(result), id, l)
		}

		m, consumed, err := parseMonkey(lines[l+1:])
		if err != nil {
			return nil, err
		}
		result = append(result, &m)

		l += consumed

		// XXX: Can we do this nicer?
		modulo *= m.divisor
	}

	return result, nil
}

func playRound(monkeys []*monkey) {
	for i, m := range monkeys {
		if debug > 2 {
			fmt.Printf("Monkey %d:\n", i)
		}
		for {
			item, ok := m.inspectItem()
			if !ok {
				break
			}

			if debug > 2 {
				fmt.Printf("  Monkey inspects an item with a worry level of %d.\n", item)
			}

			level := m.op(item)
			if debug > 2 {
				fmt.Printf("    Worry level is now %d.\n", level)
			}

			if relief {
				level /= 3
				if debug > 2 {
					fmt.Printf("    Monkey gets bored with item. Worry level is divided by 3 to %d.\n", level)
				}
			}

			var throwTo int
			if level%m.divisor == 0 {
				throwTo = m.testTrueMonkey
			} else {
				throwTo = m.testFalseMonkey
			}
			if debug > 2 {
				fmt.Printf("    Item with worry level %d is thrown to monkey %d.\n", level, throwTo)
			}

			monkeys[throwTo].catchItem(level)
		}
	}
}

func countItemInspections(monkeys []*monkey, rounds int) []int {
	for round := 1; round <= rounds; round++ {
		playRound(monkeys)

		if debug > 1 {
			fmt.Printf("After round %d, the monkeys are holding items with these worry levels:\n", round)
			for k, m := range monkeys {
				fmt.Printf("Monkey %d: %v\n", k, m.items)
			}
		}
		if debug > 0 && (round == 1 || round == 20 || round%1000 == 0) {
			fmt.Printf("== After round %d ==\n", round)
			for k, m := range monkeys {
				fmt.Printf("Monkey %d inspected items %d times.\n", k, m.inspections)
			}
		}
	}

	result := make([]int, len(monkeys))
	for k, m := range monkeys {
		result[k] = m.inspections
	}
	return result
}
