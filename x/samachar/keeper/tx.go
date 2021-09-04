package keeper

import (
	"fmt"
	"sort"
	"strconv"
	"strings"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/shravanshetty1/samachar/x/samachar/types"
)

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

func (k *Keeper) CreatePost(ctx sdk.Context, msg *types.MsgCreatePost) error {
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
		Creator:    msg.Creator,
		Id:         fmt.Sprint(postCount),
		Content:    msg.Content,
		ParentPost: msg.ParentPost,
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
	followingListRaw := string(store.Get(types.FollowingKey(msg.Creator)))
	var followingList []string
	if followingListRaw != "" {
		followingList = strings.Split(followingListRaw, ",")
	}
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
	followingListRaw := string(store.Get(types.FollowingKey(msg.Creator)))
	var followingList []string
	if followingListRaw != "" {
		followingList = strings.Split(followingListRaw, ",")
	}
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
