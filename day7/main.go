package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

const (
	LIMIT        = 100000
	DEVICE_SPACE = 70000000
	UPDATE_SPACE = 30000000
)

const (
	CMD = "$"
	CD  = "cd"
	DIR = "dir"
	LS  = "ls"

	PREV = ".."
)

type File struct {
	name string
	size int
}

func (f File) printI(indent int) {
	fmt.Printf("%s- %s (File, size=%d)\n", strings.Repeat(" ", indent), f.name, f.size)
}

type Dir struct {
	name  string
	prev  *Dir
	sdirs []*Dir
	files []File
}

func (d *Dir) moveDir(name string) *Dir {
	switch name {
	case PREV:
		return d.prev
	default:
		for _, sdir := range d.sdirs {
			if sdir.name == name {
				return sdir
			}
		}
	}

	return d
}

func (d *Dir) addSubDir(name string) {
	d.sdirs = append(d.sdirs, &Dir{
		name: name,
		prev: d,
	})
}

func (d *Dir) addFile(name string, size int) {
	d.files = append(d.files, File{
		name: name,
		size: size,
	})
}

func (d *Dir) size() int {
	var size int
	for _, f := range d.files {
		size += f.size
	}

	for _, sdir := range d.sdirs {
		size += sdir.size()
	}

	return size
}

func (d *Dir) print() {
	d.printI(0)
}

func (d *Dir) printI(indent int) {
	fmt.Printf("%s- %s (Dir)\n", strings.Repeat(" ", indent), d.name)

	indent += 2
	for _, sd := range d.sdirs {
		sd.printI(indent)
	}
	for _, f := range d.files {
		f.printI(indent)
	}
}

func main() {
	input, err := os.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}

	parentDir := parseDirs(string(input))
	parentDir.print()
	fmt.Println(partOne(parentDir))
	fmt.Println(partTwo(parentDir))
}

func partOne(parent Dir) int {
	return findSumInLimit(&parent)
}

func partTwo(parent Dir) int {
	freeSpace := DEVICE_SPACE - parent.size()
	required := UPDATE_SPACE - freeSpace

	return findSmallestEnough(&parent, required)
}

func parseDirs(input string) Dir {
	parent := Dir{
		name: "/",
		prev: nil,
	}
	current := &parent

	for _, command := range strings.Split(input, "\n")[1:] {
		switch cmd, arg := parseCmdLine(command); cmd {
		case LS:
			continue
		case CD:
			current = current.moveDir(arg)
		case DIR:
			current.addSubDir(arg)
		default:
			if size, err := strconv.Atoi(cmd); err == nil {
				current.addFile(arg, size)
			}
		}
	}

	return parent
}

func parseCmdLine(command string) (string, string) {
	switch tokens := strings.Split(command, " "); tokens[0] {
	case CMD:
		return parseCmd(tokens[1:])
	case DIR:
		return DIR, tokens[1]
	default:
		return tokens[0], tokens[1]
	}
}

func parseCmd(cmd []string) (string, string) {
	switch cmd[0] {
	case LS:
		return cmd[0], ""
	case CD:
		return cmd[0], cmd[1]
	default:
		return "", ""
	}
}

func findSumInLimit(parent *Dir) int {
	var sum int
	for _, sdir := range parent.sdirs {
		sum += findSumInLimit(sdir)

		if size := sdir.size(); size <= LIMIT {
			sum += size
		}
	}

	return sum
}

func findSmallestEnough(parent *Dir, limit int) int {
	smallest := math.MaxInt
	for _, sdir := range parent.sdirs {
		if size := sdir.size(); size < smallest && size >= limit {
			smallest = size
		}

		if size := findSmallestEnough(sdir, limit); size < smallest && size >= limit {
			smallest = size
		}
	}

	return smallest
}
