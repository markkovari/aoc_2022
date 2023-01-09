package pkg

import (
	"fmt"
	"os"
)

const (
	example = "example"
	data    = "data"
)

func createPath(ofDay int) string {
	return fmt.Sprintf("./inputs/%d/input.", ofDay)
}

func ReadInputs(ofDay int) (string, string, error) {
	prefix := createPath(ofDay)
	examplePath := fmt.Sprintf("%s%s", prefix, example)
	dataPath := fmt.Sprintf("%s%s", prefix, data)
	exampleContent, err := os.ReadFile(examplePath)
	if err != nil {
		return "", "", err
	}
	dataContent, err := os.ReadFile(dataPath)
	if err != nil {
		return "", "", err
	}
	return string(exampleContent), string(dataContent), nil
}
