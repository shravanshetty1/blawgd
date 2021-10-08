package types

// DONTCOVER

import (
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// x/blawgd module sentinel errors
var (
	ErrInputLength           = sdkerrors.Register(ModuleName, 660, "invalid input length")
	ErrInvalidId             = sdkerrors.Register(ModuleName, 661, "could not parse id")
	ErrMissingMandatoryField = sdkerrors.Register(ModuleName, 662, "field cannot be empty")
	ErrInvalidField          = sdkerrors.Register(ModuleName, 663, "field has is invalid")
)
