package cmd

import (
	day "github.com/ankon/adventofcode/2022/days/1"
	"github.com/spf13/cobra"
)

var day1Cmd = &cobra.Command{
	Use:   "day1",
	Short: "Calorie Counting",
	Run: func(cmd *cobra.Command, args []string) {
		day.Run(useSampleInput)
	},
}

func init() {
	rootCmd.AddCommand(day1Cmd)
}
