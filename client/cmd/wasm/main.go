package main

import (
	"fmt"
	"syscall/js"
)

func main() {

	document := js.Global().Get("document")
	post := document.Call("getElementById", "post")

	post.Call("addEventListener", "click", js.FuncOf(func(this js.Value, args []js.Value) interface{} {
		fmt.Println("click")

		return nil
	}))

	fmt.Println("hello")
	<-make(chan struct{})
}
