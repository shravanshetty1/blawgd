package log_test

import (
	"testing"

	"github.com/shravanshetty1/samachar/frontend-go/pkg/tendermint/libs/log"
	"github.com/stretchr/testify/require"
)

func TestNewDefaultLogger(t *testing.T) {
	testCases := map[string]struct {
		format    string
		level     string
		expectErr bool
	}{
		"invalid format": {
			format:    "foo",
			level:     log.LogLevelInfo,
			expectErr: true,
		},
		"invalid level": {
			format:    log.LogFormatJSON,
			level:     "foo",
			expectErr: true,
		},
		"valid format and level": {
			format:    log.LogFormatJSON,
			level:     log.LogLevelInfo,
			expectErr: false,
		},
	}

	for name, tc := range testCases {
		tc := tc

		t.Run(name, func(t *testing.T) {
			_, err := log.NewDefaultLogger(tc.format, tc.level, false)
			if tc.expectErr {
				require.Error(t, err)
				require.Panics(t, func() {
					_ = log.MustNewDefaultLogger(tc.format, tc.level, false)
				})
			} else {
				require.NoError(t, err)
			}
		})
	}
}
