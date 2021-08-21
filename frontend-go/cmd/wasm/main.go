package main

import (
	"context"
	"log"
	"time"

	store2 "github.com/shravanshetty1/samachar/frontend-go/pkg/tendermint/light/store"

	logging "github.com/shravanshetty1/samachar/frontend-go/pkg/tendermint/libs/log"
	"github.com/shravanshetty1/samachar/frontend-go/pkg/tendermint/light"
	"github.com/shravanshetty1/samachar/frontend-go/pkg/tendermint/light/provider"
)

const TRUSTED_HEIGHT = 13284
const TRUSTED_HASH = "C34D2576BF6CB817706D5C6FED9D9C5BBEEBFF255D33E860EC0A95B3809FD267"
const CHAIN_ID = "samachar"

func main() {

	var primary provider.Provider
	var store store2.Store
	c, err := light.NewClient(
		context.Background(),
		CHAIN_ID,
		light.TrustOptions{
			Period: 504 * time.Hour, // 21 days
			Height: TRUSTED_HEIGHT,
			Hash:   []byte(TRUSTED_HASH),
		},
		primary,
		[]provider.Provider{primary}, // NOTE: primary should not be used here
		store,
		light.Logger(logging.MustNewDefaultLogger(logging.LogFormatPlain, logging.LogLevelDebug, false)),
	)
	if err != nil {
		log.Fatal(err)
	}

	lb, err := c.Update(context.Background(), time.Now())
	if err != nil {
		log.Fatal(err)
	}

	log.Println(lb.String())
}
