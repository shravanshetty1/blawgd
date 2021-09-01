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

func (q *QueryServer) Get(ctx context.Context, request *types.GetRequest) (*types.GetResponse, error) {
	var data = make(map[string][]byte)
	var proofs = make(map[string][]byte)
	for _, key := range request.Keys {
		val, proof := q.keeper.Get(int64(request.Height), []byte(key))
		if val != nil {
			data[key] = val
			proofs[key] = proof
		}
	}

	return &types.GetResponse{
		Data:   data,
		Proofs: proofs,
	}, nil
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
		AccountInfo:    map[string]*types.AccountInfo{accountInfoKey: accountInfo},
		FollowingCount: map[string]*types.FollowingCount{followingCountKey: followingCount},
		Proofs:         map[string][]byte{accountInfoKey: proof, followingCountKey: proof2},
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
