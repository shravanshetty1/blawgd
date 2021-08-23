package main

import (
	"context"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"sort"
	"sync"
	"time"

	"google.golang.org/grpc"

	"github.com/ktr0731/grpc-web-go-client/grpcweb"

	codecTypes "github.com/shravanshetty1/samachar/frontend-go/pkg/cosmos/codec/types"
	cosmTypes "github.com/shravanshetty1/samachar/frontend-go/pkg/cosmos/types"
	"github.com/shravanshetty1/samachar/frontend-go/pkg/cosmos/types/tx/signing"

	"github.com/gogo/protobuf/proto"
	"github.com/shravanshetty1/samachar/frontend-go/pkg/cosmos/types/tx"

	"github.com/shravanshetty1/samachar/frontend-go/pkg/tendermint/crypto/secp256k1"
	tmbytes "github.com/shravanshetty1/samachar/frontend-go/pkg/tendermint/libs/bytes"
	"github.com/shravanshetty1/samachar/frontend-go/pkg/tendermint/light"
	"github.com/shravanshetty1/samachar/frontend-go/pkg/tendermint/light/provider"

	tmjson "github.com/shravanshetty1/samachar/frontend-go/pkg/tendermint/libs/json"

	"github.com/shravanshetty1/samachar/frontend-go/pkg/tendermint/types"
)

const TRUSTED_HEIGHT = 13284
const TRUSTED_HASH = `"C34D2576BF6CB817706D5C6FED9D9C5BBEEBFF255D33E860EC0A95B3809FD267"`
const CHAIN_ID = "samachar"
const COSMOS_DP = "m/44'/118'/0'/0/0"

func main() {
	//mnemonic := "voice salt fortune fork draw endless figure layer need begin trouble use cream will alpha cheese glad cook monkey used rigid better describe demise"
	//
	//privKeyByt, err := hd.Secp256k1.Derive()(mnemonic, "", COSMOS_DP)
	//if err != nil {
	//	log.Fatal(err)
	//}
	//fmt.Println(string(privKeyByt))
	//privKey := hd.Secp256k1.Generate()(privKeyByt)
	//
	//fmt.Println(privKey.PubKey().Address().String())
	//
	//err = broadcastTx(nil, 0, 0, privKey)
	//if err != nil {
	//	log.Fatal(err)
	//}

	var primary provider.Provider = NewWasmProvider()
	var store store2.Store = NewMemStore()
	hex := &tmbytes.HexBytes{}
	err := tmjson.Unmarshal([]byte(TRUSTED_HASH), &hex)
	if err != nil {
		log.Fatal(err)
	}
	trustedHash := hex.Bytes()
	c, err := light.NewClient(
		context.Background(),
		CHAIN_ID,
		light.TrustOptions{
			Period: 504 * time.Hour, // 21 days
			Height: TRUSTED_HEIGHT,
			Hash:   trustedHash,
		},
		primary,
		[]provider.Provider{primary}, // NOTE: primary should not be used here
		store,
		light.Logger(logging.MustNewDefaultLogger(logging.LogFormatPlain, logging.LogLevelInfo, false)),
	)
	if err != nil {
		log.Fatal(err)
	}

	log.Println("syncing...")
	for {
		lb, err := c.Update(context.Background(), time.Now())
		if err != nil {
			log.Fatal(err)
		}
		if lb != nil {
			log.Println(lb.String())
			log.Println("store size - " + fmt.Sprint(store.Size()))
		}

		<-time.After(time.Second)
	}
}

func broadcastTx(conn grpc.ClientConnInterface, accNum, seq uint64, privK secp256k1.PrivKey, msgs ...*codecTypes.Any) error {

	txBody := &tx.TxBody{
		Messages:                    msgs,
		Memo:                        "something",
		TimeoutHeight:               0,
		ExtensionOptions:            nil,
		NonCriticalExtensionOptions: nil,
	}

	txBodyProto, err := proto.Marshal(txBody)
	if err != nil {
		return err
	}

	signerInfo := tx.SignerInfo{
		PublicKey: &codecTypes.Any{
			TypeUrl: "/cosmos.crypto.secp256k1.PubKey",
			Value:   privK.PubKey().Bytes(),
		},
		ModeInfo: &tx.ModeInfo{Sum: &tx.ModeInfo_Single_{Single: &tx.ModeInfo_Single{Mode: signing.SignMode(1)}}},
		Sequence: seq,
	}

	authInfo := tx.AuthInfo{
		SignerInfos: []*tx.SignerInfo{&signerInfo},
		Fee: &tx.Fee{
			Amount:   cosmTypes.NewCoins(cosmTypes.NewCoin("stake", cosmTypes.NewInt(0))),
			GasLimit: 300000,
			Payer:    "",
			Granter:  "",
		},
	}

	authInfoProto, err := proto.Marshal(&authInfo)
	if err != nil {
		return err
	}

	signDoc := tx.SignDoc{
		BodyBytes:     txBodyProto,
		AuthInfoBytes: authInfoProto,
		ChainId:       CHAIN_ID,
		AccountNumber: accNum,
	}

	signDocProto, err := proto.Marshal(&signDoc)
	if err != nil {
		return err
	}

	signature, err := privK.Sign(signDocProto)
	if err != nil {
		return err
	}

	txRaw := tx.TxRaw{
		BodyBytes:     txBodyProto,
		AuthInfoBytes: authInfoProto,
		Signatures:    [][]byte{signature},
	}

	txRawProto, err := proto.Marshal(&txRaw)
	if err != nil {
		return err
	}

	txStr := tmbytes.HexBytes(txRawProto).String()

	resp, err := http.Get("http://localhost:26657/broadcast_tx_commit?tx=0x" + txStr)
	if err != nil {
		return err
	}
	respRaw, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return err
	}

	fmt.Println(string(respRaw))
	return err
}

type clWrapper struct {
	cl *grpcweb.ClientConn
}

func (c *clWrapper) Invoke(ctx context.Context, method string, args, reply interface{}, opts ...grpc.CallOption) error {
	return c.cl.Invoke(ctx, method, args, reply)
}

func (c *clWrapper) NewStream(ctx context.Context, desc *grpc.StreamDesc, method string, opts ...grpc.CallOption) (grpc.ClientStream, error) {
	return nil, nil
}

func InsertSorted(ss []int, s int) []int {
	i := sort.SearchInts(ss, s)
	ss = append(ss, 0)
	copy(ss[i+1:], ss[i:])
	ss[i] = s
	return ss
}

func DeleteSorted(ss []int, s int) []int {
	i := sort.SearchInts(ss, s)
	return append(ss[:i], ss[i+1:]...)
}

func NewMemStore() *memStore {
	return &memStore{
		mu:                 &sync.RWMutex{},
		list:               []int{},
		heightToLightBlock: make(map[int64]*types.LightBlock),
	}
}

type memStore struct {
	mu                 *sync.RWMutex
	list               []int
	heightToLightBlock map[int64]*types.LightBlock
}

func (m *memStore) SaveLightBlock(lb *types.LightBlock) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.list = InsertSorted(m.list, int(lb.Height))
	m.heightToLightBlock[lb.Height] = lb
	return nil
}

func (m *memStore) DeleteLightBlock(height int64) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	if _, ok := m.heightToLightBlock[height]; !ok {
		return nil
	}

	m.list = DeleteSorted(m.list, int(height))
	delete(m.heightToLightBlock, height)
	return nil
}

func (m *memStore) LightBlock(height int64) (*types.LightBlock, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()
	return m.heightToLightBlock[height], nil
}

func (m *memStore) LastLightBlockHeight() (int64, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()
	if len(m.list) < 1 {
		return 0, nil
	}

	return int64(m.list[len(m.list)-1]), nil
}

func (m *memStore) FirstLightBlockHeight() (int64, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()
	return int64(m.list[0]), nil
}

func (m *memStore) LightBlockBefore(height int64) (*types.LightBlock, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()
	if _, ok := m.heightToLightBlock[height]; !ok {
		return nil, fmt.Errorf("block does not exist")
	}
	i := sort.SearchInts(m.list, int(height))
	if i < 1 {
		return nil, fmt.Errorf("no blocks before this")
	}
	return m.heightToLightBlock[int64(m.list[i-1])], nil
}

func (m *memStore) Prune(s uint16) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	size := int(s)

	if len(m.list) <= size {
		return nil
	}

	for i := 0; i < len(m.list)-size; i++ {
		delete(m.heightToLightBlock, int64(m.list[i]))
	}

	m.list = m.list[size:]

	return nil
}

func (m *memStore) Size() uint16 {
	return uint16(len(m.heightToLightBlock))
}

type wasmProvider struct {
}

func (w *wasmProvider) ChainID() string {
	return CHAIN_ID
}

func NewWasmProvider() *wasmProvider {
	return &wasmProvider{}
}

const TENDERMINT_HOST = "http://localhost:26657"

// TODO make request concurrently
func (w *wasmProvider) LightBlock(ctx context.Context, height int64) (*types.LightBlock, error) {
	var commitResponse struct {
		Result struct {
			types.SignedHeader `json:"signed_header"`
			CanonicalCommit    bool `json:"canonical"`
		} `json:"result"`
	}
	var param string
	if height > 0 {
		param = "?height=" + fmt.Sprint(height)
	}
	resp, err := http.Get(TENDERMINT_HOST + "/commit" + param)
	if err != nil {
		return nil, err
	}

	body, _ := ioutil.ReadAll(resp.Body)
	err = tmjson.Unmarshal(body, &commitResponse)
	if err != nil {
		return nil, err
	}

	var validatorsResponse struct {
		Result struct {
			BlockHeight int64              `json:"block_height"`
			Validators  []*types.Validator `json:"validators"`
			// Count of actual validators in this result
			Count int `json:"count"`
			// Total number of validators
			Total int `json:"total"`
		} `json:"result"`
	}
	resp, err = http.Get(TENDERMINT_HOST + "/validators" + param)
	if err != nil {
		return nil, err
	}
	body, _ = ioutil.ReadAll(resp.Body)
	err = tmjson.Unmarshal(body, &validatorsResponse)
	if err != nil {
		return nil, err
	}

	validatorSet, err := types.ValidatorSetFromExistingValidators(validatorsResponse.Result.Validators)
	if err != nil {
		return nil, err
	}

	return &types.LightBlock{
		SignedHeader: &commitResponse.Result.SignedHeader,
		ValidatorSet: validatorSet,
	}, nil
}

// TODO confirm if this works
func (w *wasmProvider) ReportEvidence(ctx context.Context, evidence types.Evidence) error {
	evidenceJson, err := json.Marshal(evidence)
	if err != nil {
		return err
	}

	_, err = http.Get(TENDERMINT_HOST + "/broadcast_evidence?evidence=" + string(evidenceJson))
	return err
}
