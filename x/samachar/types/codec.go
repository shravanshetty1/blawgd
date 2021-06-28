package types

import (
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"

	// this line is used by starport scaffolding # 1
	"github.com/cosmos/cosmos-sdk/types/msgservice"
)

func RegisterCodec(cdc *codec.LegacyAmino) {
	// this line is used by starport scaffolding # 2
	cdc.RegisterConcrete(&MsgCreatePost{}, "createPost", nil)
	cdc.RegisterConcrete(&MsgRepost{}, "repost", nil)
	cdc.RegisterConcrete(&MsgUpdateAccountInfo{}, "updateAccountInfo", nil)
	cdc.RegisterConcrete(&MsgFollow{}, "follow", nil)
	cdc.RegisterConcrete(&MsgUnfollow{}, "unfollow", nil)
}

func RegisterInterfaces(registry cdctypes.InterfaceRegistry) {
	// this line is used by starport scaffolding # 3

	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgCreatePost{},
		&MsgRepost{},
		&MsgUpdateAccountInfo{},
	)

	msgservice.RegisterMsgServiceDesc(registry, &_Msg_serviceDesc)
}

var (
	amino     = codec.NewLegacyAmino()
	ModuleCdc = codec.NewAminoCodec(amino)
)
