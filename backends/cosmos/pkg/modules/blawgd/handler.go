package blawgd

import (
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/shravanshetty1/blawgd/backends/cosmos/pkg/modules/blawgd/keeper"
	"github.com/shravanshetty1/blawgd/backends/cosmos/pkg/modules/blawgd/types"
)

// NewHandler ...
func NewHandler(k keeper.Keeper) sdk.Handler {
	return func(ctx sdk.Context, msg sdk.Msg) (*sdk.Result, error) {
		ctx = ctx.WithEventManager(sdk.NewEventManager())

		switch msg := msg.(type) {
		case *types.MsgCreatePost:
			return handleMsgCreatePost(ctx, k, msg)
		case *types.MsgUpdateAccountInfo:
			return handleMsgUpdateAccountInfo(ctx, k, msg)
		case *types.MsgFollow:
			return handleMsgFollow(ctx, k, msg)
		case *types.MsgStopFollow:
			return handleMsgStopFollow(ctx, k, msg)
		case *types.MsgLikePost:
			return handleMsgLikePost(ctx, k, msg)
		case *types.MsgRepost:
			return handleMsgRepost(ctx, k, msg)
		default:
			errMsg := fmt.Sprintf("unrecognized %s message type: %T", types.ModuleName, msg)
			return nil, sdkerrors.Wrap(sdkerrors.ErrUnknownRequest, errMsg)
		}
	}
}
