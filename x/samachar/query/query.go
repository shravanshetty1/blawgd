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

	var postViews []*types.PostView
	for _, post := range posts {
		accountInfo := q.keeper.GetAccountInfo(sdk.UnwrapSDKContext(ctx), post.Creator)
		postView := &types.PostView{
			Creator:    accountInfo,
			Id:         post.Id,
			Content:    post.Content,
			ParentPost: post.ParentPost,
			BlockNo:    post.BlockNo,
			Metadata:   post.Metadata,
		}
		postViews = append(postViews, postView)
	}

	return &types.GetPostsByAccountResponse{Posts: postViews}, nil
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

	var postViews []*types.PostView
	for _, post := range posts {
		accountInfo := q.keeper.GetAccountInfo(sdk.UnwrapSDKContext(ctx), post.Creator)
		accountInfo.Address = post.Creator

		postView := &types.PostView{
			Creator:    accountInfo,
			Id:         post.Id,
			Content:    post.Content,
			ParentPost: post.ParentPost,
			BlockNo:    post.BlockNo,
			Metadata:   post.Metadata,
		}
		postViews = append(postViews, postView)
	}

	return &types.GetPostsByParentPostResponse{Posts: postViews}, nil
}
