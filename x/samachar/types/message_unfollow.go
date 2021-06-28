package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

var _ sdk.Msg = &MsgUnfollow{}

func NewMsgUnfollow(creator, address string) *MsgUnfollow {
	return &MsgUnfollow{
		Creator:        creator,
		AccountAddress: address,
	}
}

// Route ...
func (msg *MsgUnfollow) Route() string {
	return RouterKey
}

// Type ...
func (msg *MsgUnfollow) Type() string {
	return "Unfollow"
}

// GetSigners ...
func (msg *MsgUnfollow) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

// GetSignBytes ...
func (msg *MsgUnfollow) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

// ValidateBasic ...
func (msg *MsgUnfollow) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
