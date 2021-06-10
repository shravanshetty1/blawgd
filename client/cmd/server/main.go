package main

import (
	"bytes"
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"net/http"
	"os"

	"github.com/gorilla/mux"
)

func main() {

	router, err := NewRouter()
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Starting frontend on port 8080....")
	log.Fatal(http.ListenAndServe(":8080", router))
}

func NewRouter() (*mux.Router, error) {
	router := mux.NewRouter()
	f, err := os.Open("./client/templates/index.html")
	if err != nil {
		return nil, err
	}

	indexContent, err := ioutil.ReadAll(f)
	if err != nil {
		return nil, err
	}

	router.PathPrefix("/assets").Handler(http.StripPrefix("/assets", http.FileServer(http.Dir("./client/assets"))))
	router.Handle("/", http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(200)

		_, err := io.Copy(w, bytes.NewReader(indexContent))
		if err != nil {
			log.Println(err)
		}
	}))

	return router, nil
}
