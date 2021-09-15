module github.com/shravanshetty1/blawgd

go 1.15

require (
	github.com/NYTimes/gziphandler v1.1.1
	github.com/cosmos/cosmos-sdk v0.44.0
	github.com/cosmos/ibc-go v1.0.0-beta1
	github.com/gogo/protobuf v1.3.3
	github.com/google/go-cmp v0.5.6 // indirect
	github.com/gorilla/mux v1.8.0
	github.com/grpc-ecosystem/grpc-gateway v1.16.0
	github.com/hdevalence/ed25519consensus v0.0.0-20210430192048-0962ce16b305 // indirect
	github.com/klauspost/compress v1.11.0 // indirect
	github.com/kr/text v0.2.0 // indirect
	github.com/onsi/ginkgo v1.16.4 // indirect
	github.com/onsi/gomega v1.13.0 // indirect
	github.com/rs/cors v1.8.0 // indirect
	github.com/spf13/cast v1.3.1
	github.com/spf13/cobra v1.2.1
	github.com/spf13/pflag v1.0.5
	github.com/tendermint/tendermint v0.34.12
	github.com/tendermint/tm-db v0.6.4
	golang.org/x/crypto v0.0.0-20210813211128-0a44fdfbc16e // indirect
	golang.org/x/sys v0.0.0-20210630005230-0f9fa26af87c // indirect
	google.golang.org/genproto v0.0.0-20210617175327-b9e0b3197ced // indirect
	google.golang.org/grpc v1.38.0
)

replace google.golang.org/grpc => google.golang.org/grpc v1.33.2

replace github.com/gogo/protobuf => github.com/regen-network/protobuf v1.3.3-alpha.regen.1
