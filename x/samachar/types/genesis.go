package types

// this line is used by starport scaffolding # genesis/types/import

// DefaultIndex is the default capability global index
const DefaultIndex uint64 = 1

// DefaultGenesis returns the default Capability genesis state
func DefaultGenesis() *GenesisState {
	return &GenesisState{
		// this line is used by starport scaffolding # genesis/types/default

		// ~ 2 weeks at 2000 posts/persec
		// 2 TB
		// 2 billion
		//MaxPostCount: 2000000000,
		MaxPostCount: 2000000,
		//MaxPostCount: 6,

		// ~ 15 hours at 2000 posts/persec
		// 20 TB
		// 100 mill
		//FreePostCount: 100000000,
		FreePostCount: 100000,
		//FreePostCount: 3,
	}
}

// Validate performs basic genesis state validation returning an error upon any
// failure.
func (gs GenesisState) Validate() error {
	// this line is used by starport scaffolding # genesis/types/validate

	return nil
}
