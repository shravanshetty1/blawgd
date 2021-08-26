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

func (q *QueryServer) GetTimeline(ctxR context.Context, req *types.GetTimelineRequest) (*types.GetTimelineResponse, error) {
	ctx := sdk.UnwrapSDKContext(ctxR)
	posts := q.keeper.GetTimeline(ctx, req.Address, req.Index)

	var postViews []*types.PostView
	for _, post := range posts {
		_, accountInfo, _ := q.keeper.GetAccountInfo(0, post.Creator)
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

	return &types.GetTimelineResponse{Posts: postViews}, nil
}

func (q *QueryServer) GetFollowings(ctx context.Context, req *types.GetFollowingsRequest) (*types.GetFollowingsResponse, error) {
	return &types.GetFollowingsResponse{Addresses: q.keeper.GetFollowings(sdk.UnwrapSDKContext(ctx), req.Address).Followings}, nil
}

func (q *QueryServer) GetPost(ctx context.Context, req *types.GetPostRequest) (*types.GetPostResponse, error) {
	post, err := q.keeper.GetPost(sdk.UnwrapSDKContext(ctx), req.Id)
	if err != nil {
		return nil, err
	}

	_, accountInfo, _ := q.keeper.GetAccountInfo(0, post.Creator)
	accountInfo.Address = post.Creator

	postView := &types.PostView{
		Creator:    accountInfo,
		Id:         post.Id,
		Content:    post.Content,
		ParentPost: post.ParentPost,
		BlockNo:    post.BlockNo,
		Metadata:   post.Metadata,
	}

	return &types.GetPostResponse{Post: postView}, nil
}

func (q *QueryServer) GetPostsByAccount(ctx context.Context, req *types.GetPostsByAccountRequest) (*types.GetPostsByAccountResponse, error) {
	posts, err := q.keeper.GetPostsByAccount(sdk.UnwrapSDKContext(ctx), req.Address, req.Index, POSTS_PER_CALL)
	if err != nil {
		return nil, err
	}

	var postViews []*types.PostView
	for _, post := range posts {
		_, accountInfo, _ := q.keeper.GetAccountInfo(0, post.Creator)
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

	return &types.GetPostsByAccountResponse{Posts: postViews}, nil
}

func (q *QueryServer) GetProfileInfo(ctx context.Context, req *types.GetProfileInfoRequest) (*types.GetProfileInfoResponse, error) {
	accountInfoKey, accountInfo, proof := q.keeper.GetAccountInfo(req.Height, req.Address)
	followingCountKey, followingCount, proof2 := q.keeper.GetFollowingsCount(req.Height, req.Address)

	return &types.GetProfileInfoResponse{
		AccountInfo:    accountInfoKey,
		FollowingCount: followingCountKey,
		Data: &types.Data{
			AccountInfos:    []*types.AccountInfo{accountInfo},
			FollowingCounts: []*types.FollowingCount{followingCount},
		},
		Proofs: []*types.Proof{proof, proof2},
	}, nil
}

func (q *QueryServer) GetPostsByParentPost(ctx context.Context, req *types.GetPostsByParentPostRequest) (*types.GetPostsByParentPostResponse, error) {
	posts, err := q.keeper.GetPostsByParentPost(sdk.UnwrapSDKContext(ctx), req.ParentPost, req.Index, POSTS_PER_CALL)
	if err != nil {
		return nil, err
	}

	var postViews []*types.PostView
	for _, post := range posts {
		_, accountInfo, _ := q.keeper.GetAccountInfo(0, post.Creator)
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
