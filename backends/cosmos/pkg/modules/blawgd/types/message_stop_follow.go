package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

var _ sdk.Msg = &MsgStopFollow{}

func NewMsgStopFollow(creator, address string) *MsgStopFollow {
	return &MsgStopFollow{
		Creator: creator,
		Address: address,
	}
}

func (m *MsgStopFollow) Route() string {
	return RouterKey
}

func (m *MsgStopFollow) Type() string {
	return "StopFollow"
}

func (m *MsgStopFollow) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(m.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	_, err = sdk.AccAddressFromBech32(m.Address)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid address to follow (%s)", err)
	}

	return nil
}

func (m *MsgStopFollow) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(m)
	return sdk.MustSortJSON(bz)
}

func (m *MsgStopFollow) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(m.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}
