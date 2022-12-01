package cmd

import (
	"os"

	"github.com/spf13/cobra"
)

// If `true` uses the sample input for the day
var useSampleInput bool

var rootCmd = &cobra.Command{
	// XXX: `go build` bases the name on the module name, and changing that breaks people using `go get`.
	//      Not worth the effort right now.
	Use:   "2022",
	Short: "Run the days of AoC 2022",
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.PersistentFlags().BoolVar(&useSampleInput, "sample", true, "Use the sample input for the day")
}


