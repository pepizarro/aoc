package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"strconv"
	"unicode"
)

func readFile(filename string) ([]string, error) {

	file, err := os.Open(filename)
	if err != nil {
		return nil, errors.New("error reading file")
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var lines []string

	for scanner.Scan() {
		line := scanner.Text()
		lines = append(lines, line)
	}

	return lines, nil
}

func contains(s []rune, e rune) bool {
	for _, a := range s {
		if a == e {
			return true
		}
	}
	return false
}

func GetSymbols(lines []string) []rune {

	var symbols []rune

	for _, line := range lines {
		for _, char := range line {
			if !unicode.IsDigit(char) && !contains(symbols, char) && char != 46 {
				symbols = append(symbols, char)
			}
		}
	}
	return symbols
}

func GetFullDigit(lines *[]string, index []int) int {
	y, x := index[0], index[1]
	xN := index[1]
	var fullDigit string

	if unicode.IsDigit(rune((*lines)[y][x])) {
		fullDigit = string((*lines)[y][x])
		for {
			if xN > 0 && unicode.IsDigit(rune((*lines)[y][xN-1])) {
				fullDigit = string((*lines)[y][xN-1]) + fullDigit
				xN -= 1
			} else {
				break
			}
		}
		xN = x
		for {
			if xN < len((*lines)[0])-1 && unicode.IsDigit(rune((*lines)[y][xN+1])) {
				fullDigit = fullDigit + string((*lines)[y][xN+1])
				xN += 1
			} else {
				break
			}
		}
	}

	digit, err := strconv.Atoi(fullDigit)
	if err != nil {
		fmt.Println("Error converting to int")
		return 0
	}

	return digit
}

func GetAdjacentNumbers(lines *[]string, index []int) []int {
	var adjacentNumbers []int
	y, x := index[0], index[1]

	appendNumber := func(indexToAppend []int) {
		adjacentNumbers = append(adjacentNumbers, GetFullDigit(lines, indexToAppend))
	}
	// up
	if y > 0 && unicode.IsDigit(rune((*lines)[y-1][x])) {
		appendNumber([]int{y - 1, x})
	} else {
		// up left
		if y > 0 && x > 0 && unicode.IsDigit(rune((*lines)[y-1][x-1])) {
			appendNumber([]int{y - 1, x - 1})
		}
		// up right
		if y > 0 && x < len((*lines)[0]) && unicode.IsDigit(rune((*lines)[y-1][x+1])) {
			appendNumber([]int{y - 1, x + 1})
		}
	}

	// down
	if y < len(*lines) && unicode.IsDigit(rune((*lines)[y+1][x])) {
		adjacentNumbers = append(adjacentNumbers, GetFullDigit(lines, []int{y + 1, x}))
	} else {
		// down left
		if y < len(*lines) && x > 0 && unicode.IsDigit(rune((*lines)[y+1][x-1])) {
			appendNumber([]int{y + 1, x - 1})
		}
		// down right
		if y < len(*lines) && x < len((*lines)[0]) && unicode.IsDigit(rune((*lines)[y+1][x+1])) {
			appendNumber([]int{y + 1, x + 1})
		}
	}
	// left
	if x > 0 && unicode.IsDigit(rune((*lines)[y][x-1])) {
		appendNumber([]int{y, x - 1})
	}
	// right
	if x < len((*lines)[0]) && unicode.IsDigit(rune((*lines)[y][x+1])) {
		appendNumber([]int{y, x + 1})
	}

	// fmt.Println("Adjacent numbers: ", adjacentNumbers)
	if len(adjacentNumbers) == 2 {
		return []int{adjacentNumbers[0] * adjacentNumbers[1]}
	} else {
		return []int{}
	}
}

func GetSum(lines *[]string, symbols []rune) int {

	var numbers []int
	var sum int

	for l, line := range *lines {
		for c, char := range line {
			if char == 42 {
				numbers = append(numbers, GetAdjacentNumbers(lines, []int{l, c})...)
			}
		}
	}

	for _, n := range numbers {
		sum += n
	}

	return sum

}

func main() {

	lines, err := readFile("input.txt")
	if err != nil {
		fmt.Println(err)
		panic(err)
	}

	// symbols := []rune{'*', '=', '@', '+', '&', '%', '$', '/'}
	symbols := GetSymbols(lines)
	for _, s := range symbols {
		fmt.Print(s, " ", string(s), " | ")
	}
	fmt.Printf("\n")

	// fmt.Println(GetSum(&lines, symbols))
	fmt.Println("GetSum: ", GetSum(&lines, symbols))
}
