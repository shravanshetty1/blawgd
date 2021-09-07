package keeper

import (
	"fmt"
	"strconv"

	"github.com/cosmos/cosmos-sdk/store/prefix"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/shravanshetty1/samachar/x/samachar/types"
)

func (k *Keeper) Init(ctx sdk.Context, gen *types.GenesisState) {
	store := ctx.KVStore(k.storeKey)
	store.Set(types.MaxPostCountKey(), []byte(fmt.Sprint(gen.MaxPostCount)))
	store.Set(types.FreePostCountKey(), []byte(fmt.Sprint(gen.FreePostCount)))
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
		postIter := prefix.NewStore(store, types.PostKey("")).ReverseIterator([]byte(fmt.Sprint(1)), []byte(lastPostId))

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

			userPosts := prefix.NewStore(store, types.UserPostKey(toDeletePost.Creator, "")).Iterator(nil, nil)
			if userPosts.Valid() {
				store.Delete(types.UserPostKey(toDeletePost.Creator, string(userPosts.Key())))
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
		postIter := prefix.NewStore(store, types.PostKey("")).ReverseIterator([]byte(fmt.Sprint(1)), []byte(lastPostId))

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
				store.Delete(types.LikeKey(toFreezePostId, string(likes.Key())))
				likes.Next()
			}

			likes.Close()
			postIter.Next()
		}
		postIter.Close()
	}

	return nil
}
