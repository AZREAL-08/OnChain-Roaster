# 🔥 GaslessRoast

A decentralized roasting platform built on the Stellar blockchain using Soroban smart contracts. Post roasts, upvote the best burns, and enjoy uncensorable comedy on-chain!

## 🌟 Features

- **Post Roasts**: Share your best burns on-chain (max 280 characters)
- **Target Anyone**: Roast friends, celebrities, or concepts
- **Community Voting**: Upvote the most savage roasts
- **Immutable**: Once posted, roasts live forever on the blockchain
- **Self-Upvote Protection**: Authors can't upvote their own roasts
- **Transparent**: All roasts are publicly verifiable on-chain

## 🔗 Deployment Information

### Testnet Deployment

- **Contract Address**: `CCKUPVYOA2U53ETRFZ3BZYB5PQ7Y5WN7ODZWGFCVE3T2M2SU7UNOPK3U`
- **Network**: Stellar Testnet
- **Wasm Hash**: `5cbe0856f78f69abd4539ac5b9625442b45a45367a351e0c105192c51559248e`

### Explorer Links

- **Contract**: [View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CCKUPVYOA2U53ETRFZ3BZYB5PQ7Y5WN7ODZWGFCVE3T2M2SU7UNOPK3U)
- **Deploy Transaction**: [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/75e1d7238e6aa956a7066c1fcfbace676f20c4aa6694984bea63d997e56cf2fa)

## 📋 Prerequisites

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://www.rust-lang.org/tools/install) (for contract development)
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools#stellar-cli)
- [Freighter Wallet](https://www.freighter.app/) browser extension

## 🚀 Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/gasless-roast
cd gasless-roast
```

### 2. Backend Setup

```bash
cd contract

# Build the contract
stellar contract build

# Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/roaster_contract.wasm \
  --source-account <your-account> \
  --network testnet

# Initialize the contract
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account <your-account> \
  --network testnet \
  -- init
```

### 3. Frontend Setup

```bash
cd frontend

# Install dependencies
npm install

# Create .env file
cp .env.example .env

# Update .env with your contract ID
# VITE_CONTRACT_ID=<YOUR_CONTRACT_ID>
# VITE_RPC_URL=https://soroban-testnet.stellar.org

# Start the development server
npm run dev
```

The app will open at `http://localhost:5173`

### 4. Setup Freighter Wallet

1. Install [Freighter Wallet](https://www.freighter.app/) extension
2. Create a new wallet or import an existing one
3. Switch to **Testnet** network in Freighter settings

### 5. Fund Your Account

```bash
# Fund via Friendbot
curl "https://friendbot.stellar.org/?addr=<YOUR_ADDRESS>"
```

## 🎮 How to Use

### Posting a Roast

1. Connect your Freighter wallet
2. Enter the target (optional - can roast a person, concept, or anything)
3. Write your roast (max 280 characters)
4. Click "Post Roast"
5. Confirm the transaction in Freighter
6. Your roast is now immortalized on-chain!

### Upvoting Roasts

1. Browse the roast feed
2. Click the "🔥" button on roasts you find savage
3. Confirm the transaction
4. Note: You cannot upvote your own roasts

## 🏗️ Project Structure

```
gasless-roast/
├── contract/                   # Soroban smart contract
│   ├── src/
│   │   └── lib.rs             # Contract logic
│   ├── Cargo.toml
│   └── build.sh
│
├── frontend/                   # Vite + React frontend
│   ├── src/
│   │   ├── components/
│   │   │   ├── RoastCard.jsx      # Individual roast display
│   │   │   ├── RoastForm.jsx      # Roast creation form
│   │   │   └── WalletConnect.jsx  # Wallet connection
│   │   ├── lib/
│   │   │   ├── soroban.js         # Contract interaction
│   │   │   └── wallet.js          # Wallet utilities
│   │   ├── styles/
│   │   │   └── app.css            # Styles
│   │   ├── App.jsx
│   │   └── main.jsx
│   ├── .env.example
│   ├── package.json
│   └── vite.config.js
│
├── scripts/                    # Deployment scripts
│   ├── deploy.sh
│   └── generate-keys.sh
│
└── README.md
```

## 🔧 Smart Contract Functions

### Write Functions

- **`init()`**: Initialize the contract (one-time)
- **`post_roast(author, target, content)`**: Post a new roast
- **`upvote_roast(voter, id)`**: Upvote a roast (author cannot upvote own)

### Read Functions

- **`get_roast(id)`**: Get details of a specific roast
- **`get_roast_count()`**: Get total number of roasts posted

## 💡 Key Features Explained

### Roast Structure

Each roast contains:
- **ID**: Unique identifier
- **Author**: Address of the roaster
- **Target**: Who/what is being roasted (optional)
- **Content**: The actual roast (max 280 chars)
- **Upvotes**: Number of upvotes received

### Character Limit

Following Twitter's classic format, roasts are limited to 280 characters to encourage concise, punchy content.

### Upvote System

- Anyone can upvote any roast (except their own)
- Upvotes are on-chain and permanent
- The most upvoted roasts rise to the top

## 🛠️ Development

### Build Contract

```bash
cd contract
stellar contract build
```

### Test Contract Locally

```bash
cargo test
```

### Deploy New Contract

```bash
# Build
stellar contract build

# Install wasm
stellar contract install \
  --wasm target/wasm32-unknown-unknown/release/roaster_contract.wasm \
  --source-account <your-account> \
  --network testnet

# Deploy
stellar contract deploy \
  --wasm-hash <WASM_HASH> \
  --source-account <your-account> \
  --network testnet

# Initialize
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account <your-account> \
  --network testnet \
  -- init
```

### Frontend Development

```bash
cd frontend
npm run dev        # Start dev server
npm run build      # Build for production
npm run preview    # Preview production build
```

## 🔒 Security Considerations

- ⚠️ This is a testnet deployment - for demonstration purposes only
- All roasts are public and permanent
- Be respectful - this is for fun, not harassment
- Authors cannot upvote their own roasts
- Keep your Freighter wallet secure

## 🐛 Troubleshooting

### "Account not found" Error
Fund your account on testnet:
```bash
curl "https://friendbot.stellar.org/?addr=<YOUR_ADDRESS>"
```

### "Failed to post roast"
- Ensure content is under 280 characters
- Check you have enough XLM for transaction fees
- Verify contract ID in `.env` is correct

### Wallet Connection Issues
- Make sure Freighter is installed
- Confirm you're on Testnet network
- Try refreshing the page

### RPC Errors
- Restart dev server after changing `.env`
- Verify RPC URL: `https://soroban-testnet.stellar.org`

## 🎨 Customization

### Styling
Edit `frontend/src/styles/app.css` to customize the look and feel.

### Features to Add
- Reply threads
- User profiles
- Roast categories
- Trending roasts
- Tip system for best roasts

## 📚 Resources

- [Stellar Documentation](https://developers.stellar.org/)
- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Stellar SDK](https://github.com/stellar/js-stellar-sdk)
- [Freighter Wallet](https://www.freighter.app/)

## 🤝 Contributing

Contributions welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingRoast`)
3. Commit your changes (`git commit -m 'Add some AmazingRoast'`)
4. Push to the branch (`git push origin feature/AmazingRoast`)
5. Open a Pull Request


## 👥 Authors

- Aryan Mishra

## 🙏 Acknowledgments

- Stellar Development Foundation for Soroban
- The comedy community for inspiration
- Everyone who's ever been roasted

---

**⚠️ Disclaimer**: This is a fun demonstration project. Use responsibly and don't be mean. Remember: comedy punches up, not down!
