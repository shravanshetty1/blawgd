package types

import (
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func RegisterCodec(cdc *codec.LegacyAmino) {
	cdc.RegisterConcrete(&MsgCreatePost{}, "createPost", nil)
	cdc.RegisterConcrete(&MsgUpdateAccountInfo{}, "updateAccountInfo", nil)
	cdc.RegisterConcrete(&MsgFollow{}, "follow", nil)
	cdc.RegisterConcrete(&MsgStopFollow{}, "stopFollow", nil)
	cdc.RegisterConcrete(&MsgLikePost{}, "likePost", nil)
	cdc.RegisterConcrete(&MsgRepost{}, "repost", nil)
}

func RegisterInterfaces(registry cdctypes.InterfaceRegistry) {
	registry.RegisterImplementations((*sdk.Msg)(nil),
		&MsgCreatePost{},
		&MsgUpdateAccountInfo{},
		&MsgFollow{},
		&MsgStopFollow{},
		&MsgLikePost{},
		&MsgRepost{},
	)
}

var (
	amino     = codec.NewLegacyAmino()
	ModuleCdc = codec.NewAminoCodec(amino)
)
