package types

type NewPost struct {
	Creator      string
	Content      string
	ParentPost   string
	RepostParent *Post
}

const MAX_FOLLOWING_COUNT = 250
