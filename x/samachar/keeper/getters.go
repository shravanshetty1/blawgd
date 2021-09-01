package keeper

import (
	"math/big"

	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/shravanshetty1/samachar/x/samachar/types"
	abci "github.com/tendermint/tendermint/abci/types"
)

func (k *Keeper) GetPostsByParentPost(ctx sdk.Context, parentPost string, index, count int64) ([]*types.Post, error) {

	//postStore := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.POST_KEY))
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.SUB_POST_KEY))
	postCount := new(big.Int)
	postCount.SetString(string(store.Get(types.KeyPrefix(types.SUB_POST_COUNT_KEY+parentPost))), 10)
	postCount.Sub(postCount, big.NewInt(index))

	var posts []*types.Post
	for i := int64(0); i < count; i++ {
		if postCount.String() == "0" {
			break
		}

		postId := store.Get(types.KeyPrefix(types.SUB_POST_KEY + parentPost + "-" + postCount.String()))
		resp := k.bApp.Query(abci.RequestQuery{
			Data:   types.KeyPrefix(types.POST_KEY + types.POST_KEY + string(postId)),
			Path:   "store/samachar/key",
			Height: 0,
			Prove:  true,
		})

		postRaw := resp.Value
		//postRaw := postStore.Get(types.KeyPrefix(types.POST_KEY + string(postId)))
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

func (k *Keeper) GetPost(ctx sdk.Context, id string) (*types.Post, error) {

	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.POST_KEY))
	postRaw := store.Get(types.KeyPrefix(types.POST_KEY + id))
	var post types.Post
	err := k.cdc.Unmarshal(postRaw, &post)
	if err != nil {
		return nil, err
	}

	return &post, nil
}

func (k *Keeper) GetTimeline(ctx sdk.Context, address string, index int64) []*types.Post {
	var posts []*types.Post

	return posts
}

func (k *Keeper) GetPostsByAccount(ctx sdk.Context, address string, index, count int64) ([]*types.Post, error) {

	postStore := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.POST_KEY))
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.USER_POST_KEY))
	postCount := new(big.Int)
	postCount.SetString(string(store.Get(types.KeyPrefix(types.USER_POST_COUNT_KEY+address))), 10)
	postCount.Sub(postCount, big.NewInt(index))

	var posts []*types.Post
	for i := int64(0); i < count; i++ {
		if postCount.String() == "0" {
			break
		}

		postId := store.Get(types.KeyPrefix(types.USER_POST_KEY + address + "-" + postCount.String()))
		postRaw := postStore.Get(types.KeyPrefix(types.POST_KEY + string(postId)))
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

func (k *Keeper) Get(height int64, key []byte) ([]byte, []byte) {
	resp := k.bApp.Query(abci.RequestQuery{
		Data:   key,
		Path:   "store/samachar/key",
		Height: height,
		Prove:  true,
	})

	return resp.Value, k.cdc.MustMarshal(resp.ProofOps)
}

func (k *Keeper) GetAccountInfo(height int64, address string) (string, *types.AccountInfo, []byte) {
	key := types.AccountInfoKey(types.ACCOUNT_INFO_KEY + address)
	val, proof := k.Get(height, key)
	var accountInfo types.AccountInfo
	_ = k.cdc.Unmarshal(val, &accountInfo)
	return string(key), &accountInfo, proof
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

func (k *Keeper) GetFollowings(ctx sdk.Context, address string) types.Following {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.FOLLOWING_KEY))
	val := store.Get(types.KeyPrefix(types.FOLLOWING_KEY + address))

	var following types.Following
	k.cdc.MustUnmarshal(val, &following)
	following.Address = address

	return following
}

func (k *Keeper) GetFollowingsCount(height int64, address string) (string, *types.FollowingCount, []byte) {
	// TODO fix key
	key := types.FollowingKey(types.FOLLOWING_COUNT_KEY + address)
	val, proof := k.Get(height, key)
	var followingCount types.FollowingCount
	_ = k.cdc.Unmarshal(val, &followingCount)
	return string(key), &followingCount, proof
}
