package _2

import (
	_ "embed"
)

//go:embed sample.txt
var sampleInput string

//go:embed input.txt
var fullInput string


func pickInput(useSampleInput bool) string {
    if useSampleInput {
        return sampleInput
    } else {
        return fullInput
    }
}

func Run(useSampleInput bool) {
    pickInput(useSampleInput)
}
