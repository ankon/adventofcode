package _9

import (
	_ "embed"
	"fmt"
	"math"
	"strconv"
	"strings"

	"github.com/ankon/adventofcode/2022/days"
	"github.com/spf13/cobra"
	"golang.org/x/exp/slices"
)

//go:embed sample.txt
var sampleInput string

//go:embed input.txt
var fullInput string

var debug = false

func ConfigureCommand(cmd *cobra.Command) {
	cmd.Flags().BoolVar(&debug, "debug", false, "Enable debug output")
}

func Run(useSampleInput bool) error {
	input := days.PickInput(useSampleInput, sampleInput, fullInput)

	steps := strings.Split(strings.TrimSpace(input), "\n")
	tail, err := runSteps(steps, 1)
	if err != nil {
		return nil
	}

	fmt.Printf("Unique points in tail trail: %d\n", tail.uniquePoints())

	return nil
}

type point struct {
	x, y int
}

type trail struct {
	symbol     string
	points     []point
	bottomLeft point
	topRight   point
}

// Count the unique points in the trail
func (t *trail) uniquePoints() int {
	var uniquePoints []point
	for _, p := range t.points {
		if !slices.Contains(uniquePoints, p) {
			uniquePoints = append(uniquePoints, p)
		}
	}

	return len(uniquePoints)
}

func (t *trail) current() point {
	return t.points[len(t.points)-1]
}

func (t *trail) add(p point) {
	t.points = append(t.points, p)
	if t.bottomLeft.x > p.x {
		t.bottomLeft.x = p.x
	}
	if t.topRight.x < p.x {
		t.topRight.x = p.x
	}
	if t.bottomLeft.y > p.y {
		t.bottomLeft.y = p.y
	}
	if t.topRight.y < p.y {
		t.topRight.y = p.y
	}
}

func (t *trail) follow(other *trail) {
	// Check if the tail needs to move, if so, move it
	head := other.current()
	tail := t.current()
	touches := head.x >= tail.x-1 && head.x <= tail.x+1 && head.y >= tail.y-1 && head.y <= tail.y+1
	if !touches {
		dy := head.y - tail.y
		if dy < 0 {
			dy = -1
		} else if dy > 0 {
			dy = +1
		}
		dx := head.x - tail.x
		if dx < 0 {
			dx = -1
		} else if dx > 0 {
			dx = +1
		}
		newTail := point{tail.x + dx, tail.y + dy}
		t.add(newTail)
	}
}

func (t *trail) show(otherPoints map[string]point) {
	bottomLeft := t.bottomLeft
	topRight := t.topRight
	for _, p := range otherPoints {
		if bottomLeft.x > p.x {
			bottomLeft.x = p.x
		}
		if topRight.x < p.x {
			topRight.x = p.x
		}
		if bottomLeft.y > p.y {
			bottomLeft.y = p.y
		}
		if topRight.y < p.y {
			topRight.y = p.y
		}
	}

	c := t.current()
	for y := topRight.y; y >= bottomLeft.y; y-- {
		line := fmt.Sprintf("%3d ", y)
		for x := bottomLeft.x; x <= topRight.x; x++ {
			p := point{x, y}
			lowestK := math.MaxInt
			symbol := ""
			for s, otherPoint := range otherPoints {
				if p == otherPoint {
					k, err := strconv.Atoi(s)
					if err == nil {
						if k < lowestK {
							lowestK = k
							symbol = s
						}
					} else if s == "H" || lowestK == math.MaxInt {
						symbol = s
					}
				}
			}
			if symbol != "" {
				line += symbol
			} else if p == c {
				line += t.symbol
			} else if slices.Contains(t.points, p) {
				line += "#"
			} else {
				line += "."
			}

		}
		fmt.Println(line)
	}
}

func makeTrail(symbol string) *trail {
	return &trail{
		symbol:     symbol,
		points:     []point{{0, 0}},
		bottomLeft: point{0, 0},
		topRight:   point{5, 5},
	}
}

var directions = map[string]point{
	"R": {1, 0},
	"U": {0, 1},
	"L": {-1, 0},
	"D": {0, -1},
}

func runSteps(steps []string, knots int) (tail trail, err error) {
	start := point{0, 0}
	trails := make(map[int]*trail)
	trails[0] = makeTrail("H")
	i := 1
	for ; i < knots; i++ {
		trails[i] = makeTrail(strconv.Itoa(i))
	}
	trails[i] = makeTrail("T")
	for i, step := range steps {
		d, c, found := strings.Cut(step, " ")
		if !found {
			return trail{}, fmt.Errorf("invalid step")
		}
		stepDelta, found := directions[d]
		if !found {
			return trail{}, fmt.Errorf("invalid direction %q", d)
		}
		count, err := strconv.Atoi(c)
		if err != nil {
			return trail{}, fmt.Errorf("invalid count %q", c)
		}

		for i := 0; i < count; i++ {
			// Move the head
			head := trails[0].current()
			newHead := point{
				head.x + stepDelta.x,
				head.y + stepDelta.y,
			}
			trails[0].add(newHead)

			otherPoints := map[string]point{
				"s": start,
			}
			for t := 0; t < knots; t++ {
				otherPoints[trails[t].symbol] = trails[t].current()
			}

			if debug {
				fmt.Print("\033[H\033[2J")
				fmt.Printf("\nBefore moving tails\n")
				trails[knots].show(otherPoints)
			}

			for t := 1; t <= knots; t++ {
				trails[t].follow(trails[t-1])
			}

			if debug {
				trails[knots].show(otherPoints)
			}
		}
		fmt.Printf("Processed %d/%d steps\n", i, len(steps))
	}

	return *trails[knots], nil
}
