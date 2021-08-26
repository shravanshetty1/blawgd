package keeper

import (
	"encoding/json"
	"fmt"
	"math/big"

	"github.com/tendermint/tendermint/proto/tendermint/crypto"

	abci "github.com/tendermint/tendermint/abci/types"

	"github.com/cosmos/cosmos-sdk/baseapp"

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
		bApp     *baseapp.BaseApp
	}
)

func NewKeeper(cdc codec.Codec, storeKey, memKey sdk.StoreKey, bApp *baseapp.BaseApp) *Keeper {
	return &Keeper{
		cdc:      cdc,
		storeKey: storeKey,
		memKey:   memKey,
		bApp:     bApp,
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

	store = prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.SUB_POST_KEY))
	subPostCount := new(big.Int)
	subPostCount.SetString(string(store.Get(types.KeyPrefix(types.SUB_POST_COUNT_KEY+post.ParentPost))), 10)
	subPostCount.Add(subPostCount, big.NewInt(1))
	store.Set(types.KeyPrefix(types.SUB_POST_KEY+post.ParentPost+"-"+subPostCount.String()), []byte(post.Id))
	store.Set(types.KeyPrefix(types.SUB_POST_COUNT_KEY+post.ParentPost), []byte(subPostCount.String()))

	store = prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.USER_POST_KEY))
	userPostCount := new(big.Int)
	userPostCount.SetString(string(store.Get(types.KeyPrefix(types.USER_POST_COUNT_KEY+post.Creator))), 10)
	userPostCount.Add(userPostCount, big.NewInt(1))
	store.Set(types.KeyPrefix(types.USER_POST_KEY+post.Creator+"-"+userPostCount.String()), []byte(post.Id))
	store.Set(types.KeyPrefix(types.USER_POST_COUNT_KEY+post.Creator), []byte(userPostCount.String()))

	return nil
}

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

func (k *Keeper) Get(height int64, key []byte, val codec.ProtoMarshaler) *types.Proof {
	resp := k.bApp.Query(abci.RequestQuery{
		Data:   key,
		Path:   "store/samachar/key",
		Height: height,
		Prove:  true,
	})

	_ = k.cdc.Unmarshal(resp.Value, val)

	return GetProofFromProofOps(key, resp.ProofOps)
}

func (k *Keeper) GetAccountInfo(height int64, address string) (string, *types.AccountInfo, *types.Proof) {
	key := types.AccountInfoKey(types.ACCOUNT_INFO_KEY + address)
	var accountInfo types.AccountInfo
	proof := k.Get(height, key, &accountInfo)
	return string(key), &accountInfo, proof
}

func GetProofFromProofOps(key []byte, proofOps *crypto.ProofOps) *types.Proof {
	encodedProof, _ := json.Marshal(proofOps)
	return &types.Proof{
		Key:   string(key),
		Proof: string(encodedProof),
	}
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

func (k *Keeper) GetFollowingsCount(height int64, address string) (string, *types.FollowingCount, *types.Proof) {
	// TODO fix key
	key := types.FollowingKey(types.FOLLOWING_COUNT_KEY + address)
	var followingCount types.FollowingCount
	proof := k.Get(height, key, &followingCount)
	return string(key), &followingCount, proof
}

func (k *Keeper) StartFollowing(ctx sdk.Context, msg *types.MsgFollow) error {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.FOLLOWING_KEY))
	val := store.Get(types.KeyPrefix(types.FOLLOWING_KEY + msg.Creator))
	_, followingCount, _ := k.GetFollowingsCount(ctx.BlockHeight(), msg.Creator)

	var following types.Following
	k.cdc.MustUnmarshal(val, &following)
	following.Address = msg.Creator
	following.Followings = GetListWithoutRepeated(append(following.Followings, msg.Address))
	followingCount = &types.FollowingCount{
		Address: msg.Creator,
		Count:   uint64(len(following.Followings)),
	}

	val = k.cdc.MustMarshal(&following)
	store.Set(types.KeyPrefix(types.FOLLOWING_KEY+msg.Creator), val)
	store.Set(types.KeyPrefix(types.FOLLOWING_COUNT_KEY+msg.Creator), k.cdc.MustMarshal(followingCount))

	return nil
}

func (k *Keeper) StopFollowing(ctx sdk.Context, msg *types.MsgStopFollow) error {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.FOLLOWING_KEY))
	val := store.Get(types.KeyPrefix(types.FOLLOWING_KEY + msg.Creator))
	_, followingCount, _ := k.GetFollowingsCount(ctx.BlockHeight(), msg.Creator)

	var following types.Following
	k.cdc.MustUnmarshal(val, &following)
	following.Address = msg.Creator

	for i, v := range following.Followings {
		if v == msg.Address {
			if i == len(following.Followings)-1 {
				following.Followings = following.Followings[:i]
			} else {
				following.Followings = append(following.Followings[:i], following.Followings[i+1:]...)
			}
			break
		}
	}
	followingCount = &types.FollowingCount{
		Address: msg.Creator,
		Count:   uint64(len(following.Followings)),
	}

	val = k.cdc.MustMarshal(&following)
	store.Set(types.KeyPrefix(types.FOLLOWING_KEY+msg.Creator), val)
	store.Set(types.KeyPrefix(types.FOLLOWING_COUNT_KEY+msg.Creator), k.cdc.MustMarshal(followingCount))

	return nil
}
