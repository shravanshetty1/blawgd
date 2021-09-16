package main

import (
	"fmt"
	"log"
	"net/http"
	"net/http/httputil"
	"net/url"

	"github.com/gorilla/mux"
)

const HOST = "localhost"

func main() {

	u, err := url.Parse("http://localhost:26657")
	if err != nil {
		log.Fatal(err)
	}

	tendermintRpc := httputil.NewSingleHostReverseProxy(u)

	u, err = url.Parse("http://localhost:9091")
	if err != nil {
		log.Fatal(err)
	}

	grpcWeb := httputil.NewSingleHostReverseProxy(u)

	u, err = url.Parse("http://localhost:2342")
	if err != nil {
		log.Fatal(err)
	}

	faucet := httputil.NewSingleHostReverseProxy(u)

	u, err = url.Parse("http://localhost:2341")
	if err != nil {
		log.Fatal(err)
	}

	frontendServer := httputil.NewSingleHostReverseProxy(u)

	router := mux.NewRouter()
	router.Host("tendermint." + HOST).Subrouter().PathPrefix("/").Handler(tendermintRpc)
	router.Host("grpc." + HOST).Subrouter().PathPrefix("/").Handler(grpcWeb)
	router.Host("faucet." + HOST).Subrouter().PathPrefix("/").Handler(faucet)
	router.PathPrefix("/").Handler(frontendServer)

	fmt.Println("started reverse proxy on port 8080....")
	err = http.ListenAndServe(":8080", router)
	if err != nil {
		log.Fatal(err)
	}
}
