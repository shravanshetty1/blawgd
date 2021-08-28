package main

import (
	"io/ioutil"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"
)

const COSMOS_CONFIG_FILE = ".samachar/config/app.toml"
const TENDERMINT_CONFIG_FILE = ".samachar/config/config.toml"
const FRONTEND_CONFIG_FILE = "./frontend/wasm/src/config.rs"

func main() {

	out, err := exec.Command("./scripts/initnode/initnode.sh").CombinedOutput()
	if err != nil {
		log.Fatal(err)
	}

	log.Print(string(out))

	<-time.After(1 * time.Second)

	userHome, err := os.UserHomeDir()
	if err != nil {
		log.Fatal(err)
	}

	userHomeAbs, err := filepath.Abs(userHome)
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

	err = ioutil.WriteFile(filepath.Join(userHomeAbs, TENDERMINT_CONFIG_FILE), []byte(editedFile), 0777)
	if err != nil {
		log.Fatal(err)
	}

}
