package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

var _ sdk.Msg = &MsgUpdateAccountInfo{}

func NewMsgUpdateAccountInfo(creator string, photo string, bio string) *MsgUpdateAccountInfo {
	return &MsgUpdateAccountInfo{
		Creator: creator,
		Photo:   photo,
		Bio:     bio,
	}
}

func (m *MsgUpdateAccountInfo) Route() string {
	return RouterKey
}

func (m *MsgUpdateAccountInfo) Type() string {
	return "UpdateAccountInfo"
}

func (m *MsgUpdateAccountInfo) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(m.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}

func (m *MsgUpdateAccountInfo) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(m)
	return sdk.MustSortJSON(bz)
}

func (m *MsgUpdateAccountInfo) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(m.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}
