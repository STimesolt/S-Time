export const CONFIG = {
    // Network Configuration
    NETWORK: {
        MAINNET: 'mainnet-beta',
        TESTNET: 'testnet',
        DEVNET: 'devnet',
        LOCALNET: 'localnet',
    },

    // Program IDs
    PROGRAM_IDS: {
        TIME_ASSET_MANAGER: 'TimeAssetManager111111111111111111111111111111111',
        MARKETPLACE: 'Marketplace111111111111111111111111111111111111',
        VALIDATION: 'Validation1111111111111111111111111111111111111',
        METADATA: 'Metadata111111111111111111111111111111111111111',
    },

    // Time Slice Configuration
    TIME_SLICE: {
        MAX_DURATION: 365 * 24 * 60 * 60, // 1 year in seconds
        MIN_DURATION: 60, // 1 minute in seconds
        MAX_PRICE: 1000000000, // Maximum price in lamports
    },

    // Marketplace Configuration
    MARKETPLACE: {
        ORDER_TYPES: {
            FIXED_PRICE: 0,
            AUCTION: 1,
            DUTCH_AUCTION: 2,
        },
        ORDER_STATUS: {
            PENDING: 0,
            EXECUTED: 1,
            CANCELLED: 2,
        },
    },

    // API Configuration
    API: {
        BASE_URL: process.env.REACT_APP_API_BASE_URL || 'http://localhost:3000',
        TIMEOUT: 30000, // 30 seconds
    },
} as const;

export type Config = typeof CONFIG; 