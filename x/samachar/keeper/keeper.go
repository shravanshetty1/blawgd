package keeper

import (
	"fmt"
	"strconv"

	"github.com/google/uuid"

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
	}

	val := k.cdc.MustMarshalBinaryBare(&post)
	store.Set(key, val)

	return nil
}

func (k *Keeper) CreateRepost(ctx sdk.Context, msg *types.MsgRepost) error {
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
		Creator:      msg.Creator,
		Id:           id,
		Content:      msg.Content,
		ParentPost:   "",
		BlockNo:      ctx.BlockHeight(),
		RepostParent: msg.PostId,
	}

	val := k.cdc.MustMarshalBinaryBare(&post)
	store.Set(key, val)

	// TODO use big int
	store = prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.REPOST_COUNT_KEY))
	repostCountKey := types.KeyPrefix(types.REPOST_COUNT_KEY + msg.PostId)
	repostCountBytes := store.Get(repostCountKey)
	repostCount, _ := strconv.Atoi(string(repostCountBytes))

	repostCount += 1

	store.Set(repostCountKey, []byte(fmt.Sprint(repostCount)))

	return nil
}

func (k *Keeper) UpdateAccountInfo(ctx sdk.Context, msg *types.MsgUpdateAccountInfo) error {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.ACCOUNT_INFO_KEY))

	accountInfo := types.AccountInfo{
		Address: msg.Creator,
		Bio:     msg.Bio,
		Photo:   msg.Photo,
	}

	val := k.cdc.MustMarshalBinaryBare(&accountInfo)
	store.Set(types.KeyPrefix(types.ACCOUNT_INFO_KEY+msg.Creator), val)

	return nil
}

func (k *Keeper) Follow(ctx sdk.Context, msg *types.MsgFollow) error {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.FOLLOWING_KEY))
	key := types.KeyPrefix(types.FOLLOWING_KEY + msg.Creator)

	var following types.Following
	followingRaw := store.Get(key)
	err := k.cdc.UnmarshalBinaryBare(followingRaw, &following)
	if err != nil {
		return err
	}

	// To make sure duplicate entries are not being added, you cannot follow the same person twice
	followingMap := make(map[string]struct{}, len(following.AccountAddresses))
	for _, v := range following.AccountAddresses {
		followingMap[v] = struct{}{}
	}
	followingMap[msg.AccountAddress] = struct{}{}

	following.AccountAddresses = make([]string, len(followingMap))
	for k := range followingMap {
		following.AccountAddresses = append(following.AccountAddresses, k)
	}
	followingRaw, err = k.cdc.MarshalBinaryBare(&following)
	if err != nil {
		return err
	}
	store.Set(key, followingRaw)

	return nil
}
func (k *Keeper) Unfollow(ctx sdk.Context, msg *types.MsgUnfollow) error {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.FOLLOWING_KEY))
	key := types.KeyPrefix(types.FOLLOWING_KEY + msg.Creator)

	var following types.Following
	followingRaw := store.Get(key)
	err := k.cdc.UnmarshalBinaryBare(followingRaw, &following)
	if err != nil {
		return err
	}

	followingMap := make(map[string]struct{}, len(following.AccountAddresses))
	for _, v := range following.AccountAddresses {
		followingMap[v] = struct{}{}
	}
	delete(followingMap, msg.AccountAddress)

	following.AccountAddresses = make([]string, len(followingMap))
	for k := range followingMap {
		following.AccountAddresses = append(following.AccountAddresses, k)
	}
	followingRaw, err = k.cdc.MarshalBinaryBare(&following)
	if err != nil {
		return err
	}
	store.Set(key, followingRaw)

	return nil
}
