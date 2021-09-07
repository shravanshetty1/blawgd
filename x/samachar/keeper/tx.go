package keeper

import (
	"fmt"
	"sort"
	"strconv"
	"strings"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/shravanshetty1/samachar/x/samachar/types"
)

func (k *Keeper) CreatePost(ctx sdk.Context, newPost *types.NewPost) error {
	store := ctx.KVStore(k.storeKey)
	postCountRaw := store.Get(types.PostCountKey())
	if len(postCountRaw) < 1 {
		postCountRaw = []byte("0")
	}
	postCount, err := strconv.ParseUint(string(postCountRaw), 10, 64)
	if err != nil {
		return err
	}
	postCount += 1
	post := types.Post{
		Creator:      newPost.Creator,
		Id:           fmt.Sprint(postCount),
		Content:      newPost.Content,
		ParentPost:   newPost.ParentPost,
		RepostParent: newPost.RepostParent,
	}
	val, err := k.cdc.Marshal(&post)
	if err != nil {
		return err
	}
	store.Set(types.PostKey(post.Id), val)
	store.Set(types.PostCountKey(), []byte(fmt.Sprint(postCount)))

	parentPost, err := k.GetPost(ctx, post.ParentPost)
	if err != nil {
		return err
	}
	parentPost.CommentsCount += 1
	store.Set(types.SubpostKey(post.ParentPost, fmt.Sprint(parentPost.CommentsCount)), []byte(post.Id))
	err = k.SetPost(ctx, post.ParentPost, parentPost)
	if err != nil {
		return err
	}

	creator, err := k.GetAccountInfo(ctx, post.Creator)
	if err != nil {
		return err
	}
	creator.PostCount += 1
	store.Set(types.UserPostKey(post.Creator, fmt.Sprint(creator.PostCount)), []byte(post.Id))
	err = k.SetAccountInfo(ctx, post.Creator, creator)
	if err != nil {
		return err
	}

	return nil
}

func (k *Keeper) UpdateAccountInfo(ctx sdk.Context, msg *types.MsgUpdateAccountInfo) error {
	accountInfo, err := k.GetAccountInfo(ctx, msg.Creator)
	if err != nil {
		return err
	}
	accountInfo.Address = msg.Creator
	accountInfo.Name = msg.Name
	accountInfo.Photo = msg.Photo

	return k.SetAccountInfo(ctx, msg.Creator, accountInfo)
}

func (k *Keeper) StartFollowing(ctx sdk.Context, msg *types.MsgFollow) error {
	store := ctx.KVStore(k.storeKey)
	followingList := k.GetFollowing(ctx, msg.Creator)
	oldLen := len(followingList)

	followingList = append(followingList, msg.Address)
	followingList = GetListWithoutRepeated(followingList)

	if oldLen+1 != len(followingList) {
		return fmt.Errorf("Unexpected increase in following list length")
	}

	store.Set(types.FollowingKey(msg.Creator), []byte(strings.Join(followingList, ",")))

	creatorAccountInfo, err := k.GetAccountInfo(ctx, msg.Creator)
	if err != nil {
		return err
	}

	creatorAccountInfo.FollowingCount += 1

	err = k.SetAccountInfo(ctx, msg.Creator, creatorAccountInfo)
	if err != nil {
		return err
	}

	accountInfo, err := k.GetAccountInfo(ctx, msg.Address)
	if err != nil {
		return err
	}

	accountInfo.FollowersCount += 1

	err = k.SetAccountInfo(ctx, msg.Address, accountInfo)
	if err != nil {
		return err
	}

	return nil
}

func (k *Keeper) StopFollowing(ctx sdk.Context, msg *types.MsgStopFollow) error {
	store := ctx.KVStore(k.storeKey)
	followingList := k.GetFollowing(ctx, msg.Creator)
	oldLen := len(followingList)

	followingListMap := make(map[string]struct{})
	for _, v := range followingList {
		followingListMap[v] = struct{}{}
	}
	delete(followingListMap, msg.Address)
	var newFollowingList []string
	for k := range followingListMap {
		newFollowingList = append(newFollowingList, k)
	}
	sort.Slice(newFollowingList, func(i, j int) bool {
		return newFollowingList[i] > newFollowingList[j]
	})

	if oldLen-1 != len(newFollowingList) {
		return fmt.Errorf("Unexpected decrease in following list length")
	}

	store.Set(types.FollowingKey(msg.Creator), []byte(strings.Join(newFollowingList, ",")))

	creatorAccountInfo, err := k.GetAccountInfo(ctx, msg.Creator)
	if err != nil {
		return err
	}

	creatorAccountInfo.FollowingCount -= 1

	err = k.SetAccountInfo(ctx, msg.Creator, creatorAccountInfo)
	if err != nil {
		return err
	}

	accountInfo, err := k.GetAccountInfo(ctx, msg.Address)
	if err != nil {
		return err
	}

	accountInfo.FollowersCount -= 1

	err = k.SetAccountInfo(ctx, msg.Address, accountInfo)
	if err != nil {
		return err
	}

	return nil
}

func (k *Keeper) Like(ctx sdk.Context, msg *types.MsgLikePost) error {
	post, err := k.GetPost(ctx, msg.PostId)
	if err != nil {
		return err
	}

	post.LikeCount += msg.Amount

	sender, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return err
	}
	receiver, err := sdk.AccAddressFromBech32(post.Creator)
	if err != nil {
		return err
	}
	err = k.bKeeper.SendCoins(ctx, sender, receiver, sdk.NewCoins(sdk.NewCoin("stake", sdk.NewInt(int64(msg.Amount)))))
	if err != nil {
		return err
	}

	return k.SetPost(ctx, msg.PostId, post)
}
