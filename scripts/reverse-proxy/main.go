package main

import (
	"fmt"
	"log"
	"net/http"
	"net/http/httputil"
	"net/url"
	"os"
	"strconv"

	"github.com/rs/cors"

	"golang.org/x/crypto/acme/autocert"

	"github.com/NYTimes/gziphandler"

	"github.com/gorilla/mux"
)

const HOST = "localhost"
const PORT = 8080

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

	router.Use(gziphandler.GzipHandler, cors.AllowAll().Handler)

	env := os.Getenv("ENV")
	if env == "PROD" {

		m := autocert.Manager{
			Prompt: autocert.AcceptTOS,
			Cache:  autocert.DirCache("~/.blawgd-https"),
		}
		go http.ListenAndServe(":"+strconv.Itoa(80), m.HTTPHandler(nil))

		httpsServ := http.Server{
			Addr:      ":" + strconv.Itoa(443),
			TLSConfig: m.TLSConfig(),
			Handler:   router,
		}

		err = httpsServ.ListenAndServeTLS("", "")
		if err != nil {
			log.Fatal(err)
		}

		//fmt.Println("started reverse proxy for " + HOST)
		//certmagic.RateLimitEvents = 20000000
		//err = certmagic.HTTPS([]string{HOST, "www." + HOST, "tendermint." + HOST, "grpc." + HOST, "faucet." + HOST}, router)
		//if err != nil {
		//	log.Fatal(err)
		//}
	} else {
		fmt.Println("started reverse proxy on localhost....")
		err = http.ListenAndServe(":"+strconv.Itoa(PORT), router)
		if err != nil {
			log.Fatal(err)
		}
	}
}
