package main

import (
	"fmt"
	"github.com/markkovari/aoc_2022/pkg"
	"os"
	"strconv"
)

var associations = make(map[int]func(int) (int, int), 0)

func init() {
	associations[1] = pkg.SolveFirst
}
func main() {
	if len(os.Args) < 2 {
		panic("YOu should use a parameter")
	}
	usedArg := os.Args[1]
	dayNumber, err := strconv.ParseInt(usedArg, 10, 0)
	if err != nil || dayNumber <= 0 || dayNumber > 25 {
		panic("The parameter should be and integer and between 1 and 25")
	}
	if associations[int(dayNumber)] == nil {
		_, _ = fmt.Fprintf(os.Stderr, "The day %d is not implemented yet", dayNumber)
	} else {
		first, second := associations[int(dayNumber)](int(dayNumber))
		fmt.Fprintf(os.Stdout, "First: %d, second: %d\n", first, second)
	}

}
