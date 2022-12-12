package _12

import (
	_ "embed"
	"fmt"
	"sort"
	"strings"

	"github.com/ankon/adventofcode/2022/days"
	"github.com/spf13/cobra"
	"golang.org/x/exp/slices"
)

//go:embed sample.txt
var sampleInput string

//go:embed input.txt
var fullInput string

var debug = 0

func ConfigureCommand(cmd *cobra.Command) {
	cmd.Flags().IntVar(&debug, "debug", debug, "Enable debug output (higher numbers mean more output)")
}

func Run(useSampleInput bool) error {
	input := days.PickInput(useSampleInput, sampleInput, fullInput)
	m, start, end, err := parseHeightmap(strings.TrimSpace(input))
	if err != nil {
		return err
	}

	p := m.findPath(start, end)
	fmt.Printf("Shortest path: %d steps\n", len(p)-1)

	return nil
}

type point struct {
	x, y int
}
type heightmap [][]byte

type directionSymbol = byte

const (
	up    directionSymbol = byte('^')
	right directionSymbol = byte('>')
	down  directionSymbol = byte('v')
	left  directionSymbol = byte('<')
)

type step struct {
	point
	directionSymbol
}

type path []step

func (p *path) show(width, height int) []string {
	result := []string{}
	for y := 0; y < height; y++ {
		line := ""
		for x := 0; x < width; x++ {
			s := slices.IndexFunc(*p, func(step step) bool {
				return step.x == x && step.y == y
			})
			if s != -1 {
				line += string((*p)[s].directionSymbol)
			} else {
				line += "."
			}
		}
		result = append(result, line)
	}
	result = append(result, fmt.Sprintf("%*d", width, len(*p)))
	return result
}

func showMultiplePaths(paths []path, width, height int) {
	// XXX: +1 for the length at the end
	lines := make([][]string, height+1)
	for _, p := range paths {
		l := p.show(width, height)
		for i, tmp := range l {
			lines[i] = append(lines[i], tmp)
		}
	}

	for _, line := range lines {
		fmt.Printf("%s\n", strings.Join(line, " "))
	}
}

var directions = map[directionSymbol]point{
	up:    {0, -1},
	right: {1, 0},
	down:  {0, 1},
	left:  {-1, 0},
}

func (h *heightmap) height() int {
	return len(*h)
}
func (h *heightmap) width() int {
	return len((*h)[0])
}

func (h *heightmap) findPath(start, end point) path {
	width := h.width()
	height := h.height()

	visited := make([][]bool, height)
	for y := 0; y < height; y++ {
		visited[y] = make([]bool, width)
	}

	// Open paths, sorted by their (current) length
	open := []path{
		{step{start, startSymbol}},
	}
	visited[start.y][start.x] = true

	for len(open) > 0 {
		if debug > 0 {
			max := len(open)
			if max > 10 {
				max = 10
			}
			showMultiplePaths(open[:max], width, height)
		}

		// Take the last (shortest) one, expand the options, and then insert
		// them into the correct place
		p := open[len(open)-1]
		open = open[:len(open)-1]

		last := p[len(p)-1]
		for s, d := range directions {
			x := last.x + d.x
			if x < 0 || x >= width {
				continue
			}
			y := last.y + d.y
			if y < 0 || y >= height {
				continue
			}
			// XXX: We could go down, but should we?
			if (*h)[y][x] > (*h)[last.y][last.x]+1 {
				continue
			}
			n := point{x, y}
			if !visited[n.y][n.x] {
				visited[n.y][n.x] = true
				newPath := append(path{}, p...)
				newPath = append(newPath, step{n, s})
				newPathLen := len(newPath)

				if n == end {
					// Found a result, and we know this is the shortest; if there are multiple
					// then it will be at least as short.
					return newPath
				}

				// Find the point where we can insert the new path
				i := sort.Search(len(open), func(i int) bool {
					return newPathLen > len(open[i])
				})
				if i == len(open) {
					// Put at the end
					open = append(open, newPath)
				} else {
					// Insert _after_ the last element with the same length
					for ; i < len(open) && len(open[i]) == newPathLen; i++ {
					}
					tmp := append([]path{}, open[:i]...)
					tmp = append(tmp, newPath)
					open = append(tmp, open[i:]...)
				}
			}
		}
	}

	return nil
}

const startSymbol = byte('S')
const endSymbol = byte('E')

func parseHeightmap(input string) (m heightmap, start point, end point, err error) {
	m = [][]byte{}
	for y, line := range strings.Split(input, "\n") {
		data := []byte(line)
		x := slices.Index(data, startSymbol)
		if x != -1 {
			start = point{x, y}
			data[x] = byte('a')
		}
		x = slices.Index(data, endSymbol)
		if x != -1 {
			end = point{x, y}
			data[x] = byte('z')
		}
		m = append(m, data)
	}
	return m, start, end, nil
}
