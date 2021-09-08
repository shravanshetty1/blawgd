package keeper

import (
	"fmt"
	"strconv"

	"github.com/cosmos/cosmos-sdk/store/prefix"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/shravanshetty1/blawgd/backends/cosmos/pkg/modules/blawgd/types"
)

func (k *Keeper) Init(ctx sdk.Context, gen *types.GenesisState) {
	store := ctx.KVStore(k.storeKey)
	store.Set(types.MaxPostCountKey(), []byte(fmt.Sprint(gen.MaxPostCount)))
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

	return nil
}
