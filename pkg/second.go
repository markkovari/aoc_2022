package pkg

func SolveSecond(ofDay int) (int, int) {
	example, data, err := ReadInputs(ofDay)
	if err != nil {
		panic("UPSEY")
	}
	println("example first", first_2(example))
	println("example second", second_2(example))
	return first_2(data), second_2(data)
}

func first_2(content string) int {
	//write a solution for the first part of the day 2 of the advent of code 2022 here

	return 1
}

func second_2(content string) int {
	//write a solution for the first part of the day 2 of the advent of code 2022 here

	return 2
}
