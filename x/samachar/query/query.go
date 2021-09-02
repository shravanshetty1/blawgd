package query

import (
	context "context"

	"github.com/shravanshetty1/samachar/x/samachar/keeper"

	"github.com/shravanshetty1/samachar/x/samachar/types"
)

func NewServer(keeper keeper.Keeper) *QueryServer {
	return &QueryServer{
		keeper: keeper,
	}
}

const POSTS_PER_CALL = 30

type QueryServer struct {
	keeper keeper.Keeper
}

func (q *QueryServer) Get(ctx context.Context, request *types.GetRequest) (*types.GetResponse, error) {
	var data = make(map[string][]byte)
	var proofs = make(map[string][]byte)
	for _, key := range request.Keys {
		val, proof := q.keeper.Get(int64(request.Height), []byte(key))
		data[key] = val
		proofs[key] = proof
	}

	return &types.GetResponse{
		Data:   data,
		Proofs: proofs,
	}, nil
}

func (q *QueryServer) GetTimeline(ctxR context.Context, req *types.GetTimelineRequest) (*types.GetTimelineResponse, error) {
	return &types.GetTimelineResponse{}, nil
}
