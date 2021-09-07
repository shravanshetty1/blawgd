package keeper

import (
	"sort"
	"strings"

	"github.com/shravanshetty1/samachar/x/samachar/types"

	sdk "github.com/cosmos/cosmos-sdk/types"
	abci "github.com/tendermint/tendermint/abci/types"
)

func (k *Keeper) Get(height int64, key []byte) ([]byte, []byte) {
	resp := k.bApp.Query(abci.RequestQuery{
		Data:   key,
		Path:   "store/samachar/key",
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
