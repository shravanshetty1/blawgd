package main

import (
	"fmt"

	"github.com/cosmos/cosmos-sdk/x/bank/types"
	"github.com/gogo/protobuf/proto"
)

func main() {

	fmt.Println(proto.MessageName(&types.MsgSend{}))
}
