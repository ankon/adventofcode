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

func ConfigureCommand(cmd *cobra.Command) {
	cmd.Flags().IntVar(&debug, "debug", 0, "Enable debug output")
}

func Run(useSampleInput bool) error {
	input := days.PickInput(useSampleInput, sampleInput, fullInput)
	lines := strings.Split(strings.TrimSpace(input), "\n")

	monkeys, err := parseMonkeys(lines)
	if err != nil {
		return err
	}

	activities := countItemInspections(monkeys, 20)
	slices.Sort(activities)
	mostActivities := activities[len(activities)-1]
	secondMostActivities := activities[len(activities)-2]

	fmt.Printf("inspections: %d * %d = %d", mostActivities, secondMostActivities, mostActivities*secondMostActivities)

	return nil
}

type opFunc func(utils.UniquePrimeFactors) utils.UniquePrimeFactors
type testFunc func(utils.UniquePrimeFactors) int

type monkey struct {
	// Items the monkey has (only tracking the worry level)
	items []utils.UniquePrimeFactors

	op   opFunc
	test testFunc

	inspections int
}

func (m *monkey) inspectItem() (utils.UniquePrimeFactors, bool) {
	if len(m.items) == 0 {
		return utils.UniquePrimeFactors{}, false
	}
	result := m.items[0]
	m.items = m.items[1:]
	m.inspections++
	return result, true
}

func (m *monkey) catchItem(item utils.UniquePrimeFactors) {
	m.items = append(m.items, item)
}

const startItemsPrefix = "  Starting items: "
const opPrefix = "  Operation: new = "
const testDivisibleByPrefix = "  Test: divisible by "
const testIfTrueThrowPrefix = "    If true: throw to monkey "
const testIfFalseThrowPrefix = "    If false: throw to monkey "

func parseMonkey(lines []string) (monkey, int, error) {
	items := []utils.UniquePrimeFactors{}
	var op opFunc
	var test testFunc

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
				items = append(items, utils.Factorize(v))
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
				op = func(pf utils.UniquePrimeFactors) utils.UniquePrimeFactors {
					return utils.Factorize(pf.Value() + arg2)
				}
			case "*":
				if ss[0] != "old" {
					return monkey{}, i, fmt.Errorf("first operand must be %q for *", "old")
				}
				if ss[2] != "old" && !utils.IsPrime(arg2) {
					return monkey{}, i, fmt.Errorf("second operand must be %q or prime for *", "old")
				} 
				op = func(pf utils.UniquePrimeFactors) utils.UniquePrimeFactors {
					if ss[2] == "old" {
						for _, f := range pf {
							pf.Insert(f)
						}
					} else {
						pf.Insert(arg2)
					}
					return pf
				}
			default:
				return monkey{}, i, fmt.Errorf("unsupported operator %q", ss[1])
			}
		} else if strings.HasPrefix(line, testDivisibleByPrefix) {
			divisor, err := strconv.Atoi(line[len(testDivisibleByPrefix):])
			if err != nil {
				return monkey{}, i, fmt.Errorf("cannot parse test %q", line)
			}
			if !utils.IsPrime(divisor) {
				return monkey{}, i, fmt.Errorf("divisor %d in test must be prime", divisor)
			}
			ifTrueLine := lines[i+1]
			trueMonkey, err := strconv.Atoi(ifTrueLine[len(testIfTrueThrowPrefix):])
			if err != nil {
				return monkey{}, i, fmt.Errorf("cannot parse expected true condition %q", ifTrueLine)
			}
			ifFalseLine := lines[i+2]
			falseMonkey, err := strconv.Atoi(ifFalseLine[len(testIfFalseThrowPrefix):])
			if err != nil {
				return monkey{}, i, fmt.Errorf("cannot parse expected false condition %q", ifFalseLine)
			}
			i += 2

			test = func(item utils.UniquePrimeFactors) int {
				for _, p := range item {
					if p == divisor {
						return trueMonkey
					}
				}
				return falseMonkey
			}
		}
	}

	return monkey{items, op, test, 0}, i, nil
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
	}

	return result, nil
}

func playRound(monkeys []*monkey) {
	for i, m := range monkeys {
		if debug > 1 {
			fmt.Printf("Monkey %d:\n", i)
		}
		for {
			item, ok := m.inspectItem()
			if !ok {
				break
			}

			if debug > 1 {
				fmt.Printf("  Monkey inspects an item with a worry level of %v.\n", item)
			}

			newItem := m.op(item)
			if debug > 1 {
				fmt.Printf("    Worry level is now %v.\n", newItem)
			}

			// level /= 3
			// if debug > 1 {
			// 	fmt.Printf("    Monkey gets bored with item. Worry level is divided by 3 to %d.\n", level)
			// }

			throwTo := m.test(newItem)
			if debug > 1 {
				fmt.Printf("    Item with worry level %v is thrown to monkey %d.\n", newItem, throwTo)
			}

			monkeys[throwTo].catchItem(newItem)
		}
	}
}

func countItemInspections(monkeys []*monkey, rounds int) []int {
	for round := 0; round < rounds; round++ {
		playRound(monkeys)

		if debug > 0 {
			fmt.Printf("After round %d, the monkeys are holding items with these worry levels:\n", round+1)
			for k, m := range monkeys {
				fmt.Printf("Monkey %d: %v\n", k, m.items)
			}
		}
	}

	result := make([]int, len(monkeys))
	for i, m := range monkeys {
		result[i] = m.inspections
	}
	return result
}
