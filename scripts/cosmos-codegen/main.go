package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

func main() {
	var paths []string
	err := filepath.Walk("./frontend-go/pkg/cosmos/proto",
		func(path string, info os.FileInfo, err error) error {
			if err != nil {
				return err
			}
			if strings.HasSuffix(path, ".proto") {
				paths = append(paths, path)
			}
			return nil
		})
	if err != nil {
		log.Println(err)
	}

	fmt.Println("generating files...")
	for _, path := range paths {
		command := "-I ./frontend-go/pkg/cosmos/proto -I ./frontend-go/pkg/cosmos/third_party/proto --gocosmos_out=plugins=interfacetype+grpc,Mgoogle/protobuf/any.proto=github.com/shravanshetty1/samachar/frontend-go/pkg/cosmos/codegen/codec/types:. ./" + path
		err = exec.Command("protoc", strings.Split(command, " ")...).Run()
		if err != nil {
			log.Fatal(err)
		}
	}

	fmt.Println("placing files in appropriate folders...")
	err = os.RemoveAll("./frontend-go/pkg/cosmos/codegen")
	if err != nil {
		log.Println(1)
		log.Fatal(err)
	}
	err = os.MkdirAll("./frontend-go/pkg/cosmos/codegen", 0777)
	if err != nil {
		log.Println(1)
		log.Fatal(err)
	}
	err = exec.Command("/bin/sh", "-c", "mv -f ./github.com/cosmos/cosmos-sdk/* ./frontend-go/pkg/cosmos/codegen/").Run()
	if err != nil {
		log.Println(2)
		log.Fatal(err)
	}
	err = exec.Command("rm", strings.Split("-rf github.com", " ")...).Run()
	if err != nil {
		log.Println(3)
		log.Fatal(err)
	}

}
