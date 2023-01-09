package main

import (
	"fmt"
	"os"
	"strconv"

	"github.com/markkovari/aoc_2022/pkg"
)

var associations = make(map[int]func(int) (int, int), 0)

func init() {
	associations[1] = pkg.SolveFirst
	associations[2] = pkg.SolveSecond
}
func main() {
	if len(os.Args) < 2 {
		fmt.Println("You should use at least one parameter")
		os.Exit(1)
	}
	usedArg := os.Args[1]
	dayNumber, err := strconv.ParseInt(usedArg, 10, 0)
	if err != nil || dayNumber <= 0 || dayNumber > 25 {
		fmt.Println("The parameter should be and integer and between 1 and 25")
		os.Exit(1)
	}
	if associations[int(dayNumber)] == nil {
		_, _ = fmt.Fprintf(os.Stderr, "The day %d is not implemented yet\n", dayNumber)
	} else {
		first, second := associations[int(dayNumber)](int(dayNumber))
		fmt.Fprintf(os.Stdout, "First: %d, second: %d\n", first, second)
	}

}
