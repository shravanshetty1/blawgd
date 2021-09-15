package main

import (
	"bytes"
	"fmt"
	"io"
	"net/http"
	"os"

	"github.com/NYTimes/gziphandler"
)

type customWriter struct {
	http.ResponseWriter
	notFound bool
}

func (hrw *customWriter) WriteHeader(status int) {
	if status == 404 {
		hrw.notFound = true
		return
	}
	hrw.ResponseWriter.WriteHeader(status)
}

func (hrw *customWriter) Write(p []byte) (int, error) {
	if hrw.notFound {
		return len(p), nil
	}
	return hrw.ResponseWriter.Write(p)
}

func (hrw *customWriter) Header() http.Header {
	if hrw.notFound {
		return map[string][]string{}
	}
	return hrw.ResponseWriter.Header()
}

func main() {
	indexFile, err := os.ReadFile("./frontends/rust/server/dst/index.html")
	if err != nil {
		fmt.Println("could not read index file - " + err.Error())
		return
	}

	fmt.Println("started frontend at  port 2341...")
	err = http.ListenAndServe(":2341", gziphandler.GzipHandler(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		cw := &customWriter{
			ResponseWriter: w,
			notFound:       false,
		}
		http.FileServer(http.Dir("./frontends/rust/server/dst")).ServeHTTP(cw, r)
		if cw.notFound {
			w.Header().Set("Content-Type", "text/html; charset=utf-8")
			w.WriteHeader(200)
			io.Copy(w, bytes.NewReader(indexFile))
		}
	})))
	if err != nil {
		fmt.Println("failed to start frontend " + err.Error())
	}
}
