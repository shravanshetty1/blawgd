package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/google/uuid"
)

var _ sdk.Msg = &MsgCreatePost{}

const (
	POST_KEY = "post-"
)

func NewMsgCreatePost(creator string, content string, parentPost string) *MsgCreatePost {
	return &MsgCreatePost{
		Creator:    creator,
		Content:    content,
		ParentPost: parentPost,
		Id:         uuid.New().String(),
	}
}

// Route ...
func (msg MsgCreatePost) Route() string {
	return RouterKey
}

// Type ...
func (msg MsgCreatePost) Type() string {
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
