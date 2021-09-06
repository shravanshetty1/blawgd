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

const (
//POST_KEY            = "post-"
//POST_COUNT_KEY      = "post-count"
//SUB_POST_KEY        = "sub-post-"
//SUB_POST_COUNT_KEY  = "sub-post-count-"
//USER_POST_KEY       = "user-post-"
//USER_POST_COUNT_KEY = "user-post-count-"
//ACCOUNT_INFO_KEY    = "account-info-"
//FOLLOWING_KEY       = "following-"
//FOLLOWING_COUNT_KEY = "following-count-"
)

func UserPostKey(address, order string) []byte {
	return []byte("user-post-" + address + "-" + order)
}

func SubpostKey(parentPost, order string) []byte {
	return []byte("sub-post-" + parentPost + "-" + order)
}

func PostCountKey() []byte {
	return []byte("post-count")
}

func MaxPostCountKey() []byte {
	return []byte("max-post-count")
}

func PostKey(order string) []byte {
	return []byte("post-" + order)
}

func AccountInfoKey(address string) []byte {
	return []byte("account-info-" + address)
}

func FollowingKey(address string) []byte {
	return []byte("following-" + address)
}

func LikeKey(postId string, address string) []byte {
	return []byte("like-" + postId + "-" + address)
}
