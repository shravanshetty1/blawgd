package main

import (
	"fmt"

	"github.com/cosmos/cosmos-sdk/x/bank/types"
	"github.com/gogo/protobuf/proto"
	blawgdTypes "github.com/shravanshetty1/blawgd/backends/cosmos/pkg/modules/blawgd/types"
)

func main() {

	fmt.Println(proto.MessageName(&types.MsgSend{}))
	fmt.Println(proto.MessageName(&blawgdTypes.MsgCreatePost{}))
}
