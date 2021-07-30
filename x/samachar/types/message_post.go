package types

import (
	"math/big"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

var _ sdk.Msg = &MsgCreatePost{}

const (
	POST_KEY            = "post-"
	POST_COUNT_KEY      = "post-count"
	SUB_POST_KEY        = "sub-post-"
	SUB_POST_COUNT_KEY  = "sub-post-count-"
	USER_POST_KEY       = "user-post-"
	USER_POST_COUNT_KEY = "user-post-count-"
	ACCOUNT_INFO_KEY    = "account-info-"
	FOLLOWING_KEY       = "following-"
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

	if len(msg.Metadata) > 100 {
		return sdkerrors.Wrapf(ErrInputLength, "metadata size larger than 100 characters")
	}

	return nil
}
