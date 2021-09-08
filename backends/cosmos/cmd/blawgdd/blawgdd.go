package main

import (
	"os"

	svrcmd "github.com/cosmos/cosmos-sdk/server/cmd"
	"github.com/shravanshetty1/blawgd/backends/cosmos/pkg/app"
	"github.com/shravanshetty1/blawgd/backends/cosmos/cmd/blawgdd/cmd"
)

func main() {
	rootCmd, _ := cmd.NewRootCmd()
	if err := svrcmd.Execute(rootCmd, app.DefaultNodeHome); err != nil {
		os.Exit(1)
	}
}
