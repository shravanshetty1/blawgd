package keeper

import (
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/shravanshetty1/blawgd/backends/cosmos/pkg/modules/blawgd/types"
)

func (k *Keeper) NewAnteHandler(inner sdk.AnteHandler) sdk.AnteHandler {
	return func(ctx sdk.Context, tx sdk.Tx, simulate bool) (newCtx sdk.Context, err error) {
		msgs := tx.GetMsgs()
		for _, m := range msgs {
			switch msg := m.(type) {
			case *types.MsgRepost:
				post, err := k.GetPost(ctx, msg.PostId)
				if err != nil {
					return ctx, err
				}

				if post.Creator == "" {
					return ctx, fmt.Errorf("post does not exist")
				}

			case *types.MsgLikePost:
				post, err := k.GetPost(ctx, msg.PostId)
				if err != nil {
					return ctx, err
				}

				if post.Creator == "" {
					return ctx, fmt.Errorf("post does not exist")
				}

				if msg.Creator == post.Creator {
					return ctx, fmt.Errorf("user cannot like their own post")
				}

				addr, err := sdk.AccAddressFromBech32(msg.Creator)
				if err != nil {
					return ctx, err
				}
				balance := k.bKeeper.GetBalance(ctx, addr, "stake")
				if balance.Amount.Uint64() < msg.Amount {
					return ctx, fmt.Errorf("%v has insufficient balance to send likes - balance:%v, likes:%v", msg.Creator, balance.Amount.Uint64(), msg.Amount)
				}

			case *types.MsgFollow:
				followingList := k.GetFollowing(ctx, msg.Creator)
				var found bool
				for _, addr := range followingList {
					if addr == msg.Address {
						found = true
						break
					}
				}
				if found {
					return ctx, fmt.Errorf("already following %v", msg.Address)
				}

				maxFollowingCount, err := k.GetMaxFollowingCount(ctx)
				if err != nil {
					return ctx, err
				}

				if len(followingList) > int(maxFollowingCount) {
					return ctx, fmt.Errorf("cannot follow more then %V accounts", maxFollowingCount)
				}
			case *types.MsgStopFollow:
				followingList := k.GetFollowing(ctx, msg.Creator)
				var found bool
				for _, addr := range followingList {
					if addr == msg.Address {
						found = true
						break
					}
				}
				if !found {
					return ctx, fmt.Errorf("cannot stop following %v since your not following them", msg.Address)
				}
			case *types.MsgUpdateAccountInfo:
			case *types.MsgCreatePost:
				if msg.ParentPost != "" {
					pp, err := k.GetPost(ctx, msg.ParentPost)
					if err != nil {
						return ctx, err
					}

					if pp.Creator == "" {
						return ctx, fmt.Errorf("parent post does not exist")
					}
				}

			}
		}

		return inner(ctx, tx, simulate)
	}
}
