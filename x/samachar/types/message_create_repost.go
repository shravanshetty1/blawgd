package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

var _ sdk.Msg = &MsgRepost{}

func NewMsgRepost(creator string, content string, postId string) *MsgRepost {
	return &MsgRepost{
		Creator: creator,
		Content: content,
		PostId:  postId,
	}
}

func (m *MsgRepost) Route() string {
	return RouterKey
}

func (m *MsgRepost) Type() string {
	return "Repost"
}

func (m *MsgRepost) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(m.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}

func (m *MsgRepost) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(m)
	return sdk.MustSortJSON(bz)
}

func (m *MsgRepost) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(m.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}
