package keeper

import (
	"fmt"

	"github.com/cosmos/cosmos-sdk/store/prefix"

	"github.com/tendermint/tendermint/libs/log"

	"github.com/cosmos/cosmos-sdk/codec"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/shravanshetty1/samachar/x/samachar/types"
)

type (
	Keeper struct {
		cdc      codec.Marshaler
		storeKey sdk.StoreKey
		memKey   sdk.StoreKey
	}
)

func NewKeeper(cdc codec.Marshaler, storeKey, memKey sdk.StoreKey) *Keeper {
	return &Keeper{
		cdc:      cdc,
		storeKey: storeKey,
		memKey:   memKey,
	}
}

func (k Keeper) Logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", types.ModuleName))
}

func (k *Keeper) CreatePost(ctx sdk.Context, msg *types.MsgCreatePost) error {

	post := types.Post{
		Creator:    msg.Creator,
		Id:         msg.Id,
		Content:    msg.Content,
		ParentPost: msg.ParentPost,
		BlockNo:    ctx.BlockHeight(),
	}

	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.POST_KEY))
	key := types.KeyPrefix(types.POST_KEY + post.Id)
	val := k.cdc.MustMarshalBinaryBare(&post)
	store.Set(key, val)

	return nil
}
