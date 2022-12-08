package _7

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"

	"github.com/ankon/adventofcode/2022/days"
)

//go:embed sample.txt
var sampleInput string

//go:embed input.txt
var fullInput string

const totalDiskSpace = 70000000
const requiredUnusedDiskSpace = 30000000

type ftype int32

const (
	file ftype = iota
	dir
)

type direntry struct {
	ftype ftype
	size  int
	name  string

	children []direntry
	parent   *direntry
}

func readDir(parent *direntry, lines []string) ([]direntry, int, error) {
	entries := []direntry{}
	i := 0
	for i = 0; i < len(lines); i++ {
		if lines[i] == "" {
			break
		}

		s, name, found := strings.Cut(lines[i], " ")
		if !found {
			return []direntry{}, -1, fmt.Errorf("invalid ls output %q", lines[i])
		}
		if s == "$" {
			break
		}

		var ftype ftype
		var size int
		if s == "dir" {
			ftype = dir
		} else {
			ftype = file
			tmp, err := strconv.Atoi(s)
			if err != nil {
				return []direntry{}, -1, err
			}
			size = tmp
		}
		entries = append(entries, direntry{
			ftype,
			size,
			name,
			[]direntry{},
			parent,
		})
	}

	return entries, i, nil
}

func loadFS(input string) (root direntry, err error) {
	root = direntry{
		ftype:    dir,
		size:     0,
		name:     "/",
		children: []direntry{},
	}
	cwd := &root
	lines := strings.Split(input, "\n")
	for i := 0; i < len(lines); i++ {
		cmd := lines[i]
		if cmd == "" {
			continue
		}

		p := strings.Split(cmd, " ")
		if p[0] != "$" {
			return direntry{}, fmt.Errorf("command expected")
		}

		switch p[1] {
		case "cd":
			name := p[2]
			if name == "/" {
				cwd = &root
			} else if name == ".." {
				cwd = cwd.parent
			} else {
				var ncwd *direntry
				for i := 0; i < len(cwd.children); i++ {
					if cwd.children[i].name == name {
						ncwd = &cwd.children[i]
						break
					}
				}
				if ncwd == nil {
					return direntry{}, fmt.Errorf("no such directory %q", name)
				}
				cwd = ncwd
			}
		case "ls":
			entries, count, err := readDir(cwd, lines[i+1:])
			if err != nil {
				return direntry{}, err
			}
			cwd.children = append(cwd.children, entries...)
			i += count
		default:
			return direntry{}, fmt.Errorf("unknown command %q", p[0])
		}
	}

	return root, nil
}

func printDir(de *direntry, indent int) {
	fmt.Printf("%*s- %s\n", indent, "", de.name)
	for _, child := range de.children {
		switch child.ftype {
		case dir:
			printDir(&child, indent+2)
		case file:
			fmt.Printf("%*s- %s (size=%d)\n", indent+2, "", child.name, child.size)
		default:
			panic("huh?")
		}
	}
}

func calculateDirSizes(de *direntry) int {
	size := 0
	for i := 0; i < len(de.children); i++ {
		child := &de.children[i]
		if child.ftype == dir {
			size += calculateDirSizes(child)
		} else {
			size += child.size
		}
	}
	de.size = size
	return size
}

func findSumOfDirsWithSizeLessThan100K(de *direntry) int {
	result := 0

	for i := 0; i < len(de.children); i++ {
		child := &de.children[i]
		if child.ftype == file {
			continue
		}

		if child.size < 100000 {
			result += child.size
		}
		result += findSumOfDirsWithSizeLessThan100K(child)
	}
	return result
}

func findSmallestDirectoryWithSizeLargerThan(de *direntry, minSize int) *direntry {
	var result *direntry
	for i := 0; i < len(de.children); i++ {
		child := &de.children[i]
		if child.ftype == file {
			continue
		}
		if child.size < minSize {
			// Too small, and won't get bigger
			continue
		}

		// This directory might work, see whether we can just
		// delete a part of it.
		better := findSmallestDirectoryWithSizeLargerThan(child, minSize)
		if better == nil {
			// Nope, the whole thing needs to go.
			better = child
		}
		if result == nil || result.size > better.size {
			result = better
		}
	}
	return result
}

func Run(useSampleInput bool) error {
	input := days.PickInput(useSampleInput, sampleInput, fullInput)
	root, err := loadFS(input)
	if err != nil {
		return err
	}
	printDir(&root, 0)

	totalSize := calculateDirSizes(&root)
	fmt.Printf("total size %d\n", totalSize)

	sum := findSumOfDirsWithSizeLessThan100K(&root)
	fmt.Printf("sum of sizes of dirs < 100000: %d\n", sum)

	requiredCleanupSize := requiredUnusedDiskSpace - (totalDiskSpace - totalSize)
	toDelete := findSmallestDirectoryWithSizeLargerThan(&root, requiredCleanupSize)
	if toDelete == nil {
		return fmt.Errorf("cannot find directory to delete to free up %d", requiredCleanupSize)
	}
	fmt.Printf("delete %q, frees up %d\n", toDelete.name, toDelete.size)

	return nil
}