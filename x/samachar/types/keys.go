package types

const (
	// ModuleName defines the module name
	ModuleName = "samachar"

	// StoreKey defines the primary module store key
	StoreKey = ModuleName

	// RouterKey is the message route for slashing
	RouterKey = ModuleName

	// QuerierRoute defines the module's query routing key
	QuerierRoute = ModuleName

	// MemStoreKey defines the in-memory store key
	MemStoreKey = "mem_capability"
)

func KeyPrefix(p string) []byte {
	return []byte(p)
}

func AccountInfoKey(address string) []byte {
	return []byte(ACCOUNT_INFO_KEY + address)
}

func FollowingCountKey(address string) []byte {
	return []byte(FOLLOWING_COUNT_KEY + address)
}

func FollowingKey(address string) []byte {
	return []byte(FOLLOWING_KEY + address)
}
