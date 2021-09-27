package keeper

import (
	"fmt"
	"sort"
	"strconv"
	"strings"

	"github.com/shravanshetty1/blawgd/backends/cosmos/pkg/modules/blawgd/types"

	sdk "github.com/cosmos/cosmos-sdk/types"
	abci "github.com/tendermint/tendermint/abci/types"
)

func (k *Keeper) Get(height int64, key []byte) ([]byte, []byte) {
	resp := k.bApp.Query(abci.RequestQuery{
		Data:   key,
		Path:   "store/blawgd/key",
		Height: height,
		Prove:  true,
	})

	return resp.Value, k.cdc.MustMarshal(resp.ProofOps)
}

func (k *Keeper) GetFollowing(ctx sdk.Context, address string) []string {
	store := ctx.KVStore(k.storeKey)
	followingListRaw := string(store.Get(types.FollowingKey(address)))
	var followingList []string
	if followingListRaw != "" {
		followingList = strings.Split(followingListRaw, ",")
	}
	return followingList
}

func (k *Keeper) GetMaxFollowingCount(ctx sdk.Context) (uint64, error) {
	store := ctx.KVStore(k.storeKey)
	maxFollowingCountRaw := store.Get(types.MaxFollowingCountKey())
	maxFollowingCount, err := strconv.ParseUint(string(maxFollowingCountRaw), 10, 64)
	if err != nil {
		return 0, err
	}

	return maxFollowingCount, nil
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

	sort.Slice(newList, func(i, j int) bool {
		return newList[i] > newList[j]
	})

	return newList
}

func (k *Keeper) GetPostsByAccount(height, page, perPage int64, address string) (map[string][]byte, map[string][]byte, error) {
	var data = make(map[string][]byte)
	var proofs = make(map[string][]byte)
	key := types.AccountInfoKey(address)
	accountInfoRaw, proof := k.Get(height, key)
	data[string(key)] = accountInfoRaw
	proofs[string(key)] = proof

	var accountInfo types.AccountInfo
	err := k.cdc.Unmarshal(accountInfoRaw, &accountInfo)
	if err != nil {
		return nil, nil, err
	}

	min, max, err := pagination(1, int64(accountInfo.PostCount), page, perPage)
	if err != nil {
		return nil, nil, err
	}

	var userPostKeys [][]byte
	for i := max; i >= min; i-- {
		userPostKeys = append(userPostKeys, types.UserPostKey(address, fmt.Sprint(i)))
	}

	var postIds []string
	for _, key := range userPostKeys {
		postId, proof := k.Get(height, key)
		data[string(key)] = postId
		proofs[string(key)] = proof
		postIds = append(postIds, string(postId))
	}

	d, p, err := k.GetPosts(height, postIds)
	if err != nil {
		return nil, nil, err
	}

	CopyMap(d, data)
	CopyMap(p, proofs)

	return data, proofs, nil
}

func (k *Keeper) GetPostsByParentPost(height, page, perPage int64, parentPostId string) (map[string][]byte, map[string][]byte, error) {
	var data = make(map[string][]byte)
	var proofs = make(map[string][]byte)
	d, p, err := k.GetPosts(height, []string{parentPostId})
	if err != nil {
		return nil, nil, err
	}
	CopyMap(d, data)
	CopyMap(p, proofs)
	key := types.PostKey(parentPostId)
	parentPostRaw := d[string(key)]

	var parentPost types.Post
	err = k.cdc.Unmarshal(parentPostRaw, &parentPost)
	if err != nil {
		return nil, nil, err
	}

	min, max, err := pagination(1, int64(parentPost.CommentsCount), page, perPage)
	if err != nil {
		return nil, nil, err
	}

	var subPostKeys [][]byte
	for i := max; i >= min; i-- {
		subPostKeys = append(subPostKeys, types.SubpostKey(parentPostId, fmt.Sprint(i)))
	}

	var postIds []string
	for _, key := range subPostKeys {
		postId, proof := k.Get(height, key)
		data[string(key)] = postId
		proofs[string(key)] = proof

		postIds = append(postIds, string(postId))
	}

	d, p, err = k.GetPosts(height, postIds)
	if err != nil {
		return nil, nil, err
	}

	CopyMap(d, data)
	CopyMap(p, proofs)

	return data, proofs, nil
}

func (k *Keeper) GetPosts(height int64, postIds []string) (map[string][]byte, map[string][]byte, error) {
	var data = make(map[string][]byte)
	var proofs = make(map[string][]byte)

	for _, postId := range postIds {
		postKey := types.PostKey(fmt.Sprint(postId))
		postRaw, proof := k.Get(height, postKey)
		data[string(postKey)] = postRaw
		proofs[string(postKey)] = proof

		var post types.Post
		err := k.cdc.Unmarshal(postRaw, &post)
		if err != nil {
			return nil, nil, err
		}

		key := types.AccountInfoKey(post.Creator)
		postCreator, proof := k.Get(height, key)
		data[string(key)] = postCreator
		proofs[string(key)] = proof
		if post.RepostParent != nil {
			key := types.AccountInfoKey(post.RepostParent.Creator)
			repostCreator, proof := k.Get(height, key)
			data[string(key)] = repostCreator
			proofs[string(key)] = proof
		}
	}

	return data, proofs, nil
}

func (k *Keeper) GetTimeline(height, page, perPage int64, address string) (map[string][]byte, map[string][]byte, error) {
	var data = make(map[string][]byte)
	var proofs = make(map[string][]byte)
	key := types.FollowingKey(address)
	followingListRaw, proof := k.Get(height, key)
	data[string(key)] = followingListRaw
	proofs[string(key)] = proof

	followingList := strings.Split(string(followingListRaw), ",")

	for _, address := range followingList {
		d, p, err := k.GetPostsByAccount(height, page, perPage, address)
		if err != nil {
			return nil, nil, err
		}
		CopyMap(d, data)
		CopyMap(p, proofs)
	}

	return data, proofs, nil
}

func CopyMap(src, dst map[string][]byte) {
	for k, v := range src {
		dst[k] = v
	}
}

func pagination(absMin, absMax, page, perPage int64) (int64, int64, error) {
	max := absMax
	min := absMin
	if max < (perPage * (page - 1)) {
		return 0, 0, fmt.Errorf("page number %v to high, not enough pages in collection", page)
	}
	if max > perPage {
		max = max - (perPage * (page - 1))
		if max > perPage {
			min = max + 1 - perPage
		}
	}

	return min, max, nil
}
