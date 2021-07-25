package query

import (
	context "context"

	sdk "github.com/cosmos/cosmos-sdk/types"

	"github.com/shravanshetty1/samachar/x/samachar/keeper"

	"github.com/shravanshetty1/samachar/x/samachar/types"
)

type QueryServer struct {
	keeper keeper.Keeper
}

func NewServer(keeper keeper.Keeper) *QueryServer {
	return &QueryServer{
		keeper: keeper,
	}
}

const POSTS_PER_CALL = 30

func (q *QueryServer) GetPosts(ctx context.Context, req *types.GetPostsRequest) (*types.GetPostsResponse, error) {

	posts, err := q.keeper.GetPosts(sdk.UnwrapSDKContext(ctx), req.Index, POSTS_PER_CALL)
	if err != nil {
		return nil, err
	}

	return &types.GetPostsResponse{Posts: posts}, nil
}
