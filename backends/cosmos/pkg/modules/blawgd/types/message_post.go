package types

import (
	"math/big"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

var _ sdk.Msg = &MsgCreatePost{}

func NewMsgCreatePost(creator, content, parentPost string) *MsgCreatePost {
	return &MsgCreatePost{
		Creator:    creator,
		Content:    content,
		ParentPost: parentPost,
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

	if msg.ParentPost != "" {
		buf := new(big.Int)
		if _, valid := buf.SetString(msg.ParentPost, 10); !valid {
			return sdkerrors.Wrapf(ErrInvalidId, "could not parse parent post")
		}
	}

	if msg.Content == "" {
		return sdkerrors.Wrapf(ErrMissingMandatoryField, "content cannot be empty")
	}

	if len(msg.Content) > 280 {
		return sdkerrors.Wrapf(ErrInputLength, "post size larger than 280 characters")
	}

	return nil
}
