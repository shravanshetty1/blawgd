package keeper

import (
	"fmt"
	"math/big"

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
	postCount := new(big.Int)
	postCount.SetString(string(store.Get(types.KeyPrefix(types.POST_COUNT_KEY))), 10)
	postCount.Add(postCount, big.NewInt(1))

	post := types.Post{
		Creator:    msg.Creator,
		Id:         postCount.String(),
		Content:    msg.Content,
		ParentPost: msg.ParentPost,
		BlockNo:    ctx.BlockHeight(),
		Metadata:   msg.Metadata,
	}

	val := k.cdc.MustMarshal(&post)
	store.Set(types.KeyPrefix(types.POST_KEY+postCount.String()), val)
	store.Set(types.KeyPrefix(types.POST_COUNT_KEY), []byte(postCount.String()))

	return nil
}

func (k *Keeper) GetPosts(ctx sdk.Context, index, count int64) ([]*types.Post, error) {

	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.POST_KEY))
	postCount := new(big.Int)
	postCount.SetString(string(store.Get(types.KeyPrefix(types.POST_COUNT_KEY))), 10)
	postCount.Sub(postCount, big.NewInt(index))

	var posts []*types.Post
	for i := int64(0); i < count; i++ {
		if postCount.String() == "0" {
			break
		}

		postRaw := store.Get(types.KeyPrefix(types.POST_KEY + postCount.String()))
		var post types.Post
		err := k.cdc.Unmarshal(postRaw, &post)
		if err != nil {
			return nil, err
		}
		posts = append(posts, &post)

		postCount.Sub(postCount, big.NewInt(1))
	}

	return posts, nil
}

func (k *Keeper) UpdateAccountInfo(ctx sdk.Context, msg *types.MsgUpdateAccountInfo) error {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.ACCOUNT_INFO_KEY))

	accountInfo := types.AccountInfo{
		Address:  msg.Creator,
		Name:     msg.Name,
		Photo:    msg.Photo,
		Metadata: msg.Metadata,
	}

	val := k.cdc.MustMarshal(&accountInfo)
	store.Set(types.KeyPrefix(types.ACCOUNT_INFO_KEY+msg.Creator), val)

	return nil
}

func (k *Keeper) GetAccountInfo(ctx sdk.Context, address string) *types.AccountInfo {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.ACCOUNT_INFO_KEY))
	accountInfoRaw := store.Get(types.KeyPrefix(types.ACCOUNT_INFO_KEY + address))

	var accountInfo types.AccountInfo
	k.cdc.MustUnmarshal(accountInfoRaw, &accountInfo)
	return &accountInfo
}

func GetListWithoutRepeated(list []string) []string {
	uniqList := make(map[string]struct{}, len(list))
	for _, v := range list {
		uniqList[v] = struct{}{}
	}

	var newList []string
	for k := range uniqList {
		newList = append(newList, k)
	}

	return newList
}

func (k *Keeper) StartFollowing(ctx sdk.Context, msg *types.MsgFollow) error {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.FOLLOWING_KEY))
	val := store.Get(types.KeyPrefix(types.FOLLOWING_KEY + msg.Creator))

	var following types.Following
	k.cdc.MustUnmarshal(val, &following)
	following.Address = msg.Creator

	following.Followings = GetListWithoutRepeated(append(following.Followings, msg.Address))

	val = k.cdc.MustMarshal(&following)
	store.Set(types.KeyPrefix(types.FOLLOWING_KEY+msg.Creator), val)

	return nil
}

func (k *Keeper) StopFollowing(ctx sdk.Context, msg *types.MsgStopFollow) error {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.FOLLOWING_KEY))
	val := store.Get(types.KeyPrefix(types.FOLLOWING_KEY + msg.Creator))

	var following types.Following
	k.cdc.MustUnmarshal(val, &following)
	following.Address = msg.Creator

	for i, v := range following.Followings {
		if v == msg.Address {
			following.Followings = append(following.Followings[:i], following.Followings[i+1:]...)
			break
		}
	}

	val = k.cdc.MustMarshal(&following)
	store.Set(types.KeyPrefix(types.FOLLOWING_KEY+msg.Creator), val)

	return nil
}
