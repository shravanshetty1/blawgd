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
	if len(args) != 3 {
		log.Fatal("unexpected number of args", args)
	}

	homeDir := args[1]
	mnemonic := args[2]

	out, err := exec.Command("./backends/cosmos/scripts/network/genesis-validator/init/main.sh", homeDir, mnemonic).CombinedOutput()
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

	editedFile := strings.Replace(string(b), "enable-unsafe-cors = false", "enable-unsafe-cors = true", -1)

	err = ioutil.WriteFile(filepath.Join(userHomeAbs, COSMOS_CONFIG_FILE), []byte(editedFile), 0777)
	if err != nil {
		log.Fatal(err)
	}

	b, err = ioutil.ReadFile(filepath.Join(userHomeAbs, TENDERMINT_CONFIG_FILE))
	if err != nil {
		log.Fatal(err)
	}

	editedFile = strings.Replace(string(b), "cors_allowed_origins = []", `cors_allowed_origins = ["*"]`, -1)
	editedFile = strings.Replace(editedFile, `timeout_commit = "5s"`, `timeout_commit = "1s"`, -1)

	err = ioutil.WriteFile(filepath.Join(userHomeAbs, TENDERMINT_CONFIG_FILE), []byte(editedFile), 0777)
	if err != nil {
		log.Fatal(err)
	}

}