package query

import (
	context "context"

	sdk "github.com/cosmos/cosmos-sdk/types"

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

func (q *QueryServer) GetPostsByAccount(ctx context.Context, req *types.GetPostsByAccountRequest) (*types.GetPostsByAccountResponse, error) {
	posts, err := q.keeper.GetPostsByAccount(sdk.UnwrapSDKContext(ctx), req.Address, req.Index, POSTS_PER_CALL)
	if err != nil {
		return nil, err
	}

	return &types.GetPostsByAccountResponse{Posts: posts}, nil
}

func (q *QueryServer) GetAccountInfo(ctx context.Context, req *types.GetAccountInfoRequest) (*types.GetAccountInfoResponse, error) {
	accountInfo := q.keeper.GetAccountInfo(sdk.UnwrapSDKContext(ctx), req.Address)

	return &types.GetAccountInfoResponse{AccountInfo: accountInfo}, nil
}

func (q *QueryServer) GetPostsByParentPost(ctx context.Context, req *types.GetPostsByParentPostRequest) (*types.GetPostsByParentPostResponse, error) {
	posts, err := q.keeper.GetPostsByParentPost(sdk.UnwrapSDKContext(ctx), req.ParentPost, req.Index, POSTS_PER_CALL)
	if err != nil {
		return nil, err
	}

	return &types.GetPostsByParentPostResponse{Posts: posts}, nil
}
