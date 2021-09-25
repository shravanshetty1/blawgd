package query

import (
	context "context"

	"github.com/shravanshetty1/blawgd/backends/cosmos/pkg/modules/blawgd/keeper"

	"github.com/shravanshetty1/blawgd/backends/cosmos/pkg/modules/blawgd/types"
)

func NewServer(keeper keeper.Keeper) *QueryServer {
	return &QueryServer{
		keeper: keeper,
	}
}

type QueryServer struct {
	keeper keeper.Keeper
}

func (q *QueryServer) GetPosts(ctx context.Context, req *types.GetPostsRequest) (*types.GetResponse, error) {
	data, proofs, err := q.keeper.GetPosts(req.Height, req.PostIds)
	if err != nil {
		return nil, err
	}

	return &types.GetResponse{
		Data:   data,
		Proofs: proofs,
	}, nil
}

func (q *QueryServer) GetPostsByParentPost(ctx context.Context, req *types.GetPostsByParentPostRequest) (*types.GetResponse, error) {
	data, proofs, err := q.keeper.GetPostsByParentPost(req.Height, req.Page, req.PerPage, req.ParentPost)
	if err != nil {
		return nil, err
	}

	return &types.GetResponse{
		Data:   data,
		Proofs: proofs,
	}, nil
}

func (q *QueryServer) GetPostsByAccount(ctx context.Context, req *types.GetPostsByAccountRequest) (*types.GetResponse, error) {
	data, proofs, err := q.keeper.GetPostsByAccount(req.Height, req.Page, req.PerPage, req.Address)
	if err != nil {
		return nil, err
	}

	return &types.GetResponse{
		Data:   data,
		Proofs: proofs,
	}, nil
}

func (q *QueryServer) GetTimeline(ctx context.Context, req *types.GetTimelineRequest) (*types.GetResponse, error) {
	data, proofs, err := q.keeper.GetTimeline(req.Height, req.Page, req.PerPage, req.Address)
	if err != nil {
		return nil, err
	}

	return &types.GetResponse{
		Data:   data,
		Proofs: proofs,
	}, nil
}

func (q *QueryServer) mustEmbedUnimplementedQueryServer() {
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
