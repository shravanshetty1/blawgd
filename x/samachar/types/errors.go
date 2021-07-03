package types

// DONTCOVER

import (
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// x/samachar module sentinel errors
var (
	ErrInputLength = sdkerrors.Register(ModuleName, 660, "invalid input length")
)
