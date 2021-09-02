package keeper

import (
	"sort"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/shravanshetty1/samachar/x/samachar/types"
	abci "github.com/tendermint/tendermint/abci/types"
)

func (k *Keeper) GetTimeline(ctx sdk.Context, address string, index int64) []*types.Post {
	var posts []*types.Post

	return posts
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
