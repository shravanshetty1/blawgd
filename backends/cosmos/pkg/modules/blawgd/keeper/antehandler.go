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
			switch m.(type) {
			case *types.MsgRepost:
				msg, _ := m.(*types.MsgRepost)
				post, err := k.GetPost(ctx, msg.PostId)
				if err != nil {
					return ctx, err
				}

				if post.Creator == "" {
					return ctx, fmt.Errorf("post does not exist")
				}

				if post.RepostParent != "" {
					return ctx, fmt.Errorf("cannot repost a repost")
				}

			case *types.MsgLikePost:
				msg, _ := m.(*types.MsgLikePost)
				post, err := k.GetPost(ctx, msg.PostId)
				if err != nil {
					return ctx, err
				}

				if post.Creator == "" {
					return ctx, fmt.Errorf("post does not exist")
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
				msg, _ := m.(*types.MsgFollow)
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

				if len(followingList) > types.MAX_FOLLOWING_COUNT {
					return ctx, fmt.Errorf("cannot follow more than %v accounts", types.MAX_FOLLOWING_COUNT)
				}
			case *types.MsgStopFollow:
				msg, _ := m.(*types.MsgStopFollow)
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
				msg, _ := m.(*types.MsgCreatePost)
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
