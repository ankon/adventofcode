package days

func PickInput(useSampleInput bool, sampleInput string, fullInput string) string {
	if useSampleInput {
		return sampleInput
	} else {
		return fullInput
	}
}
