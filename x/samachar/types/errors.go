package types

// DONTCOVER

import (
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// x/samachar module sentinel errors
var (
	ErrInputLength           = sdkerrors.Register(ModuleName, 660, "invalid input length")
	ErrInvalidId             = sdkerrors.Register(ModuleName, 661, "could not parse id")
	ErrMissingMandatoryField = sdkerrors.Register(ModuleName, 662, "field cannot be empty")
)
