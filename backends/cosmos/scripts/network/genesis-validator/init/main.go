package main

import (
	"io/ioutil"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

const COSMOS_CONFIG_FILE = "config/app.toml"
const TENDERMINT_CONFIG_FILE = "config/config.toml"

func main() {
	args := os.Args
	if len(args) != 4 {
		log.Fatal("unexpected number of args", args)
	}

	homeDir := args[1]
	mnemonic := args[2]
	faucet := args[3]

	out, err := exec.Command("./backends/cosmos/scripts/network/genesis-validator/init/main.sh", homeDir, mnemonic, faucet).CombinedOutput()
	if err != nil {
		log.Fatal(err)
	}

	log.Print(string(out))

	userHomeAbs, err := filepath.Abs(homeDir)
	if err != nil {
		log.Fatal(err)
	}

	b, err := ioutil.ReadFile(filepath.Join(userHomeAbs, COSMOS_CONFIG_FILE))
	if err != nil {
		log.Fatal(err)
	}

	editedFile := strings.Replace(string(b), `pruning = "default"`, `pruning = "custom"`, -1)
	editedFile = strings.Replace(editedFile, `pruning-keep-recent = "0"`, `pruning-keep-recent = "10"`, -1)
	editedFile = strings.Replace(editedFile, `pruning-interval = "0"`, `pruning-interval = "10"`, -1)

	err = ioutil.WriteFile(filepath.Join(userHomeAbs, COSMOS_CONFIG_FILE), []byte(editedFile), 0777)
	if err != nil {
		log.Fatal(err)
	}

	b, err = ioutil.ReadFile(filepath.Join(userHomeAbs, TENDERMINT_CONFIG_FILE))
	if err != nil {
		log.Fatal(err)
	}

	editedFile = strings.Replace(string(b), `timeout_commit = "5s"`, `timeout_commit = "1s"`, -1)
	editedFile = strings.Replace(editedFile, `create_empty_blocks = true`, `create_empty_blocks = false`, -1)

	err = ioutil.WriteFile(filepath.Join(userHomeAbs, TENDERMINT_CONFIG_FILE), []byte(editedFile), 0777)
	if err != nil {
		log.Fatal(err)
	}

}
