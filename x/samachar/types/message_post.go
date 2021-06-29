package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

var _ sdk.Msg = &MsgCreatePost{}

const (
	POST_KEY         = "post-"
	ACCOUNT_INFO_KEY = "account-info-"
)

func NewMsgCreatePost(creator, content, parentPost, metadata string) *MsgCreatePost {
	return &MsgCreatePost{
		Creator:    creator,
		Content:    content,
		ParentPost: parentPost,
		Metadata:   metadata,
	}
}

// Route ...
func (msg *MsgCreatePost) Route() string {
	return RouterKey
}

// Type ...
func (msg *MsgCreatePost) Type() string {
	return "CreatePost"
}

// GetSigners ...
func (msg *MsgCreatePost) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

// GetSignBytes ...
func (msg *MsgCreatePost) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

// ValidateBasic ...
func (msg *MsgCreatePost) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
