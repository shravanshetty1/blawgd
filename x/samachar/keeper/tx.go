package keeper

import (
	"fmt"
	"sort"
	"strconv"
	"strings"

	"github.com/cosmos/cosmos-sdk/store/prefix"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/shravanshetty1/samachar/x/samachar/types"
)

func (k *Keeper) Init(ctx sdk.Context, gen *types.GenesisState) {
	store := ctx.KVStore(k.storeKey)
	store.Set(types.MaxPostCountKey(), []byte(fmt.Sprint(gen.MaxPostCount)))
	store.Set(types.FreePostCountKey(), []byte(fmt.Sprint(gen.FreePostCount)))
}

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

func (k *Keeper) EndBlock(ctx sdk.Context) error {
	store := ctx.KVStore(k.storeKey)
	postCountRaw := store.Get(types.PostCountKey())
	if len(postCountRaw) < 1 {
		postCountRaw = []byte("0")
	}
	postCount, err := strconv.ParseUint(string(postCountRaw), 10, 64)
	if err != nil {
		return err
	}

	maxPostCountRaw := store.Get(types.MaxPostCountKey())
	maxPostCount, err := strconv.ParseUint(string(maxPostCountRaw), 10, 64)
	if err != nil {
		return err
	}

	// delete old post
	if postCount > maxPostCount {
		lastPostId := fmt.Sprint(postCount - maxPostCount)
		postIter := prefix.NewStore(store, types.PostKey("")).Iterator(types.PostKey(lastPostId), nil)

		for postIter.Valid() {
			toDeletePostId := string(postIter.Key())
			toDeletePost, err := k.GetPost(ctx, toDeletePostId)
			if err != nil {
				return err
			}
			// return if post does not exist
			if toDeletePost.Creator == "" {
				break
			}

			store.Delete(types.PostKey(toDeletePostId))

			// Subposts will get cleared automatically
			for i := uint64(1); i < toDeletePost.CommentsCount+1; i++ {
				store.Delete(types.SubpostKey(toDeletePostId, fmt.Sprint(i)))
			}

			userPosts := prefix.NewStore(store, types.UserPostKey(toDeletePost.Creator, "")).ReverseIterator(nil, nil)
			if userPosts.Valid() {
				store.Delete(userPosts.Key())
			}

			userPosts.Close()
			postIter.Next()
		}
		postIter.Close()
	}

	freePostCountRaw := store.Get(types.FreePostCountKey())
	freePostCount, err := strconv.ParseUint(string(freePostCountRaw), 10, 64)
	if err != nil {
		return err
	}

	// freeze posts
	if postCount > freePostCount {
		lastPostId := fmt.Sprint(postCount - freePostCount)
		postIter := prefix.NewStore(store, types.PostKey("")).Iterator(types.PostKey(lastPostId), nil)

		for postIter.Valid() {
			toFreezePostId := string(postIter.Key())
			toFreezePost, err := k.GetPost(ctx, toFreezePostId)
			if err != nil {
				return err
			}
			// return if post does not exist
			if toFreezePost.Creator == "" {
				break
			}
			// break if post already frozen
			if toFreezePost.Frozen {
				break
			}

			toFreezePost.Frozen = true

			err = k.SetPost(ctx, toFreezePostId, toFreezePost)
			if err != nil {
				return err
			}

			likes := prefix.NewStore(store, types.LikeKey(toFreezePostId, "")).Iterator(nil, nil)
			for likes.Valid() {
				store.Delete(likes.Key())
				likes.Next()
			}

			likes.Close()
			postIter.Next()
		}
		postIter.Close()
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

func (k *Keeper) Like(ctx sdk.Context, msg *types.MsgLikePost) error {
	store := ctx.KVStore(k.storeKey)
	val := store.Get(types.LikeKey(msg.PostId, msg.Creator))
	if string(val) == "1" {
		return nil
	}

	post, err := k.GetPost(ctx, msg.PostId)
	if err != nil {
		return err
	}

	if post.Frozen {
		return fmt.Errorf("cannot like frozen post")
	}

	store.Set(types.LikeKey(msg.PostId, msg.Creator), []byte("1"))

	post.LikeCount += 1

	sender, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return err
	}
	reciever, err := sdk.AccAddressFromBech32(post.Creator)
	if err != nil {
		return err
	}
	err = k.bKeeper.SendCoins(ctx, sender, reciever, sdk.NewCoins(sdk.NewCoin("stake", sdk.NewInt(int64(msg.Tip)))))
	if err != nil {
		return err
	}

	return k.SetPost(ctx, msg.PostId, post)
}

func (k *Keeper) Unlike(ctx sdk.Context, msg *types.MsgUnlikePost) error {
	store := ctx.KVStore(k.storeKey)
	val := store.Get(types.LikeKey(msg.PostId, msg.Creator))
	if string(val) != "1" {
		return nil
	}

	post, err := k.GetPost(ctx, msg.PostId)
	if err != nil {
		return err
	}

	if post.Frozen {
		return fmt.Errorf("cannot unlike frozen post")
	}

	store.Delete(types.LikeKey(msg.PostId, msg.Creator))

	post.LikeCount -= 1

	return k.SetPost(ctx, msg.PostId, post)
}
