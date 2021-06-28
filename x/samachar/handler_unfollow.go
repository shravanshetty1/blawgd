package samachar

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/shravanshetty1/samachar/x/samachar/keeper"
	"github.com/shravanshetty1/samachar/x/samachar/types"
)

func handleMsgUnfollow(ctx sdk.Context, k keeper.Keeper, msg *types.MsgUnfollow) (*sdk.Result, error) {
	err := k.Unfollow(ctx, msg)
	if err != nil {
		return nil, err
	}

	return &sdk.Result{Events: ctx.EventManager().ABCIEvents()}, nil
}
