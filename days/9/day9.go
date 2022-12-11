package _9

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"

	"github.com/ankon/adventofcode/2022/days"
	"golang.org/x/exp/slices"
)

//go:embed sample.txt
var sampleInput string

//go:embed input.txt
var fullInput string

const debug = false

func Run(useSampleInput bool) error {
	input := days.PickInput(useSampleInput, sampleInput, fullInput)

	steps := strings.Split(strings.TrimSpace(input), "\n")
	_, trail, err := runSteps(steps)
	if err != nil {
		return nil
	}

	fmt.Printf("Unique points in tail trail: %d\n", trail.uniquePoints())

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

func (t *trail) show(otherPoints map[string]point) {
	c := t.current()
	for y := t.topRight.y; y >= t.bottomLeft.y; y-- {
		line := fmt.Sprintf("%3d ", y)
	nextX:
		for x := t.bottomLeft.x; x <= t.topRight.x; x++ {
			p := point{x, y}
			for k, otherPoint := range otherPoints {
				if p == otherPoint {
					line += k
					continue nextX
				}
			}
			if p == c {
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

func makeTrail(symbol string) trail {
	return trail{
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

func runSteps(steps []string) (headtrail trail, tailtrail trail, err error) {
	start := point{0, 0}
	headtrail = makeTrail("H")
	tailtrail = makeTrail("T")
	for i, step := range steps {
		d, c, found := strings.Cut(step, " ")
		if !found {
			return headtrail, tailtrail, fmt.Errorf("invalid step")
		}
		stepDelta, found := directions[d]
		if !found {
			return headtrail, tailtrail, fmt.Errorf("invalid direction %q", d)
		}
		count, err := strconv.Atoi(c)
		if err != nil {
			return headtrail, tailtrail, fmt.Errorf("invalid count %q", c)
		}

		for i := 0; i < count; i++ {
			// Move the head
			newHead := point{
				headtrail.current().x + stepDelta.x,
				headtrail.current().y + stepDelta.y,
			}
			headtrail.add(newHead)

			otherPoints := map[string]point{
				"s": start,
				"H": newHead,
			}

			if debug {
				fmt.Print("\033[H\033[2J")
				fmt.Printf("\nBefore moving tail\n")
				tailtrail.show(otherPoints)
			}

			// Check if the tail needs to move, if so, move it
			tail := tailtrail.current()
			touches := newHead.x >= tail.x-1 && newHead.x <= tail.x+1 && newHead.y >= tail.y-1 && newHead.y <= tail.y+1
			if !touches {
				dy := newHead.y - tail.y
				if dy < 0 {
					dy = -1
				} else if dy > 0 {
					dy = +1
				}
				dx := newHead.x - tail.x
				if dx < 0 {
					dx = -1
				} else if dx > 0 {
					dx = +1
				}
				newTail := point{tail.x + dx, tail.y + dy}
				tailtrail.add(newTail)
			}

			if debug {
				tailtrail.show(otherPoints)
			}
		}
		fmt.Printf("Processed %d/%d steps\n", i, len(steps))
	}

	return headtrail, tailtrail, nil
}
