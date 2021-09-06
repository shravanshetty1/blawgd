package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

var _ sdk.Msg = &MsgUnlikePost{}

func NewMsgUnlikePost(creator, postId string) *MsgUnlikePost {
	return &MsgUnlikePost{
		Creator: creator,
		PostId:  postId,
	}
}

// Route ...
func (msg *MsgUnlikePost) Route() string {
	return RouterKey
}

// Type ...
func (msg *MsgUnlikePost) Type() string {
	return "UnlikePost"
}

// GetSigners ...
func (msg *MsgUnlikePost) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

// GetSignBytes ...
func (msg *MsgUnlikePost) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

// ValidateBasic ...
func (msg *MsgUnlikePost) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}

	return nil
}
