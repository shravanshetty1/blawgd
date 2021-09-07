package types

import (
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

var _ sdk.Msg = &MsgLikePost{}

func NewMsgLikePost(creator, postId string) *MsgLikePost {
	return &MsgLikePost{
		Creator: creator,
		PostId:  postId,
	}
}

// Route ...
func (msg *MsgLikePost) Route() string {
	return RouterKey
}

// Type ...
func (msg *MsgLikePost) Type() string {
	return "LikePost"
}

// GetSigners ...
func (msg *MsgLikePost) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

// GetSignBytes ...
func (msg *MsgLikePost) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

// ValidateBasic ...
func (msg *MsgLikePost) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}

	if msg.Tip == 0 {
		return fmt.Errorf("tip cannot be 0")
	}

	return nil
}
