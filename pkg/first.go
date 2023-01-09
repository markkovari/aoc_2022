package pkg

import (
	"sort"
	"strconv"
	"strings"
)

func SolveFirst(ofDay int) (int, int) {
	example, data, err := ReadInputs(ofDay)
	if err != nil {
		panic("UPSEY")
	}
	println("example first", first_1(example))
	println("example second", second_1(example))
	return first_1(data), second_1(data)
}

func first_1(content string) int {
	islands := strings.Split(content, "\n\n")
	maxValue := 0
	for _, island := range islands {
		value := getSum(strings.Split(island, "\n"))
		if value > maxValue {
			maxValue = value
		}
	}
	return maxValue
}

func second_1(content string) int {
	islands := strings.Split(content, "\n\n")
	values := make([]int, 0)
	for _, island := range islands {
		value := getSum(strings.Split(island, "\n"))
		values = append(values, value)
	}
	sort.Sort(sort.Reverse(sort.IntSlice(values)))
	return values[0] + values[1] + values[2]
}

func getSum(values []string) int {
	sum := 0
	for _, value := range values {
		asNum, err := strconv.ParseInt(value, 10, 0)
		if err == nil {
			sum += int(asNum)
		}
	}
	return sum
}
