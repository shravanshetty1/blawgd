package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

var _ sdk.Msg = &MsgFollow{}

func NewMsgFollow(creator, address string) *MsgFollow {
	return &MsgFollow{
		Creator:        creator,
		AccountAddress: address,
	}
}

// Route ...
func (msg *MsgFollow) Route() string {
	return RouterKey
}

// Type ...
func (msg *MsgFollow) Type() string {
	return "Follow"
}

// GetSigners ...
func (msg *MsgFollow) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

// GetSignBytes ...
func (msg *MsgFollow) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

// ValidateBasic ...
func (msg *MsgFollow) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
