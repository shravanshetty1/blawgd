package keeper

import (
	"math/big"

	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/shravanshetty1/samachar/x/samachar/types"
)

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
