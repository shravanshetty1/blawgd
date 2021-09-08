package keeper

import (
	"fmt"

	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"

	"github.com/cosmos/cosmos-sdk/baseapp"

	"github.com/tendermint/tendermint/libs/log"

	"github.com/cosmos/cosmos-sdk/codec"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/shravanshetty1/blawgd/backends/cosmos/pkg/modules/blawgd/types"
)

type (
	Keeper struct {
		cdc      codec.Codec
		storeKey sdk.StoreKey
		memKey   sdk.StoreKey
		bApp     *baseapp.BaseApp
		bKeeper  bankkeeper.Keeper
	}
)

func NewKeeper(cdc codec.Codec, storeKey, memKey sdk.StoreKey, bApp *baseapp.BaseApp, b bankkeeper.Keeper) *Keeper {
	return &Keeper{
		cdc:      cdc,
		storeKey: storeKey,
		memKey:   memKey,
		bApp:     bApp,
		bKeeper:  b,
	}
}

func (k Keeper) Logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", types.ModuleName))
}

func (k *Keeper) GetAccountInfo(ctx sdk.Context, address string) (types.AccountInfo, error) {
	store := ctx.KVStore(k.storeKey)
	accountInfoRaw := store.Get(types.AccountInfoKey(address))
	var accountInfo types.AccountInfo
	err := k.cdc.Unmarshal(accountInfoRaw, &accountInfo)
	if err != nil {
		return types.AccountInfo{}, err
	}
	return accountInfo, nil
}

func (k *Keeper) SetAccountInfo(ctx sdk.Context, address string, accountInfo types.AccountInfo) error {
	store := ctx.KVStore(k.storeKey)
	accountInfo.Address = address
	accountInfoRaw, err := k.cdc.Marshal(&accountInfo)
	if err != nil {
		return err
	}
	store.Set(types.AccountInfoKey(address), accountInfoRaw)
	return nil
}

func (k *Keeper) GetPost(ctx sdk.Context, id string) (types.Post, error) {
	store := ctx.KVStore(k.storeKey)
	postRaw := store.Get(types.PostKey(id))
	var post types.Post
	err := k.cdc.Unmarshal(postRaw, &post)
	if err != nil {
		return types.Post{}, err
	}
	return post, nil
}

func (k *Keeper) SetPost(ctx sdk.Context, id string, post types.Post) error {
	store := ctx.KVStore(k.storeKey)
	post.Id = id
	postRaw, err := k.cdc.Marshal(&post)
	if err != nil {
		return err
	}
	store.Set(types.PostKey(id), postRaw)
	return nil
}
