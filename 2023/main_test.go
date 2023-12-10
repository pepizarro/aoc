package main

// import (
// 	"fmt"
// 	"testing"

// 	"github.com/stretchr/testify/assert"
// )

// var bag map[string]int

// func TestCheckHand(t *testing.T) {
// 	bag = map[string]int{
// 		"red":   12,
// 		"green": 13,
// 		"blue":  14,
// 	}

// 	t.Run("Valid Hand", func(t *testing.T) {
// 		assert := assert.New(t)
// 		input := "1 red, 2 green, 6 blue"
// 		expected := true

// 		out := checkHand(input, bag)
// 		assert.Equal(out, expected, "Hand should be possible")
// 	})

// 	t.Run("Invalid Hand", func(t *testing.T) {
// 		assert := assert.New(t)
// 		input := "8 green, 6 blue, 20 red"
// 		expected := false

// 		out := checkHand(input, bag)
// 		assert.Equal(out, expected, "Hand should be impossible")
// 	})
// }

// func TestCheckGame(t *testing.T) {
// 	bag = map[string]int{
// 		"red":   12,
// 		"green": 13,
// 		"blue":  14,
// 	}

// 	t.Run("Possible Game", func(t *testing.T) {
// 		assert := assert.New(t)
// 		input := "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
// 		expectedId, expectedPossibility := 1, true

// 		outId, outPossibility := checkGame(input, bag)
// 		assert.Equal(outId, expectedId, "Game ID should match")
// 		assert.Equal(outPossibility, expectedPossibility, "Possibility should match")

// 	})

// 	t.Run("Impossible Game", func(t *testing.T) {
// 		assert := assert.New(t)
// 		input := "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
// 		expectedId, expectedPossibility := 0, false

// 		outId, outPossibility := checkGame(input, bag)
// 		assert.Equal(outId, expectedId, "Game ID should match")
// 		assert.Equal(outPossibility, expectedPossibility, "Possibility should match")

// 	})
// }

// func TestCheckAllGames(t *testing.T) {
// 	bag = map[string]int{
// 		"red":   12,
// 		"green": 13,
// 		"blue":  14,
// 	}
// 	assert := assert.New(t)
// 	input, err := readFile("example.txt")
// 	if err != nil {
// 		fmt.Println(err)
// 	}
// 	expected := 8
// 	out := checkAllGames(input, bag)
// 	assert.Equal(out, expected, "Different count of ID's")

// }
