package keeper

import (
	"fmt"

	"github.com/google/uuid"

	"github.com/cosmos/cosmos-sdk/store/prefix"

	"github.com/tendermint/tendermint/libs/log"

	"github.com/cosmos/cosmos-sdk/codec"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/shravanshetty1/samachar/x/samachar/types"
)

type (
	Keeper struct {
		cdc      codec.Codec
		storeKey sdk.StoreKey
		memKey   sdk.StoreKey
	}
)

func NewKeeper(cdc codec.Codec, storeKey, memKey sdk.StoreKey) *Keeper {
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

	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.POST_KEY))

	var id string
	var key []byte
	for {
		id = uuid.NewString()
		key = types.KeyPrefix(types.POST_KEY + id)
		if len(store.Get(key)) == 0 {
			break
		}
	}

	post := types.Post{
		Creator:    msg.Creator,
		Id:         id,
		Content:    msg.Content,
		ParentPost: msg.ParentPost,
		BlockNo:    ctx.BlockHeight(),
		Metadata:   msg.Metadata,
	}

	val := k.cdc.MustMarshal(&post)
	store.Set(key, val)

	return nil
}

func (k *Keeper) UpdateAccountInfo(ctx sdk.Context, msg *types.MsgUpdateAccountInfo) error {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.ACCOUNT_INFO_KEY))

	accountInfo := types.AccountInfo{
		Address: msg.Creator,
		Bio:     msg.Bio,
		Photo:   msg.Photo,
	}

	val := k.cdc.MustMarshal(&accountInfo)
	store.Set(types.KeyPrefix(types.ACCOUNT_INFO_KEY+msg.Creator), val)

	return nil
}
