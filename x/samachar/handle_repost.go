package samachar

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/shravanshetty1/samachar/x/samachar/keeper"
	"github.com/shravanshetty1/samachar/x/samachar/types"
)

func handleMsgRepost(ctx sdk.Context, k keeper.Keeper, msg *types.MsgRepost) (*sdk.Result, error) {
	err := k.CreatePost(ctx, &types.NewPost{
		Creator:      msg.Creator,
		RepostParent: msg.PostId,
	})

	return &sdk.Result{Events: ctx.EventManager().ABCIEvents()}, err
}
