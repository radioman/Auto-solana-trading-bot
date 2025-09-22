# 🚀 Solana PumpPortal Trading Bot

<div align="center">

![Solana](https://img.shields.io/badge/Solana-9945FF?style=for-the-badge&logo=solana&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![Node.js](https://img.shields.io/badge/Node.js-43853D?style=for-the-badge&logo=node.js&logoColor=white)
![WebSocket](https://img.shields.io/badge/WebSocket-010101?style=for-the-badge&logo=socket.io&logoColor=white)

**Advanced automated trading bot for Solana tokens using PumpPortal WebSocket feeds**

[Features](#-features) • [Installation](#-installation) • [Configuration](#-configuration) • [Usage](#-usage) • [API](#-api) • [Contributing](#-contributing)

</div>

---

## ✨ Features

### 🔥 Core Trading Features
- **Real-time Token Detection**: Monitors PumpPortal WebSocket for new token launches
- **Automated Trading**: Executes buy/sell orders based on market conditions
- **Risk Management**: Built-in stop-loss and take-profit mechanisms
- **Position Tracking**: Monitors active positions and PnL in real-time
- **Multi-token Support**: Handles multiple concurrent token positions

### 🛡️ Safety & Security
- **Slippage Protection**: Configurable slippage tolerance for trades
- **Liquidity Checks**: Validates minimum liquidity before trading
- **Error Handling**: Comprehensive error handling and recovery
- **Rate Limiting**: Built-in rate limiting to prevent API abuse

### 📊 Monitoring & Analytics
- **Real-time Logging**: Detailed logging with configurable levels
- **Telegram Notifications**: Get notified of trades, errors, and status updates
- **Portfolio Tracking**: Track total PnL and trade statistics
- **Performance Metrics**: Monitor success rates and profitability

### 🔧 Technical Features
- **TypeScript**: Fully typed for better development experience
- **Modular Architecture**: Clean, maintainable code structure
- **WebSocket Reconnection**: Automatic reconnection with exponential backoff
- **Transaction Optimization**: Optimized for Solana's transaction model

---

## 🚀 Installation

### Prerequisites

- **Node.js** (v18 or higher)
- **npm** or **yarn**
- **Solana Wallet** with SOL for trading
- **Telegram Bot** (optional, for notifications)

### Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/radioman/Auto-solana-trading-bot.git
   cd Auto-solana-trading-bot
   ```

2. **Install dependencies**
   ```bash
   npm install
   # or
   yarn install
   ```

3. **Set up environment variables**
   ```bash
   cp env.example .env
   ```

4. **Configure your environment**
   Edit `.env` file with your settings:
   ```env
   # Required
   PRIVATE_KEY=your_base58_private_key_here
   
   # Optional
   RPC_ENDPOINT=https://api.mainnet-beta.solana.com
   BUY_AMOUNT_SOL=0.01
   MAX_CONCURRENT_TRADES=5
   TELEGRAM_BOT_TOKEN=your_telegram_bot_token
   TELEGRAM_CHAT_ID=your_telegram_chat_id
   ```

5. **Build and run**
   ```bash
   # Development
   npm run dev
   
   # Production
   npm run build
   npm start
   ```

---

## ⚙️ Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `PRIVATE_KEY` | Base58 encoded Solana private key | - | ✅ |
| `RPC_ENDPOINT` | Solana RPC endpoint | `https://api.mainnet-beta.solana.com` | ❌ |
| `COMMITMENT_LEVEL` | Transaction commitment level | `confirmed` | ❌ |
| `BUY_AMOUNT_SOL` | SOL amount per trade | `0.01` | ❌ |
| `MAX_CONCURRENT_TRADES` | Maximum concurrent positions | `5` | ❌ |
| `STOP_LOSS_PERCENTAGE` | Stop loss percentage | `20` | ❌ |
| `TAKE_PROFIT_PERCENTAGE` | Take profit percentage | `50` | ❌ |
| `TELEGRAM_BOT_TOKEN` | Telegram bot token | - | ❌ |
| `TELEGRAM_CHAT_ID` | Telegram chat ID | - | ❌ |
| `SOLANA_VIBE_STATION_API_KEY` | API key for token metadata | - | ❌ |

### Trading Configuration

You can modify trading parameters in `src/constants/index.ts`:

```typescript
export const BUY_AMOUNT_SOL = 0.01;           // SOL per trade
export const MAX_CONCURRENT_TRADES = 5;       // Max positions
export const STOP_LOSS_PERCENTAGE = 20;       // Stop loss %
export const TAKE_PROFIT_PERCENTAGE = 50;     // Take profit %
```

---

## 🎯 Usage

### Basic Usage

```bash
# Start the bot
npm run dev

# Or build and run
npm run build
npm start
```

### Advanced Usage

```typescript
import { SolanaTradingBot } from './src/index';

const bot = new SolanaTradingBot();

// Start trading
await bot.start();

// Get current status
const status = bot.getStatus();
console.log(status);

// Stop trading
await bot.stop();
```

### Monitoring

The bot provides real-time monitoring through:

1. **Console Logs**: Detailed logging with timestamps
2. **Telegram Notifications**: Real-time alerts and updates
3. **Status Endpoint**: Programmatic status checking

---

## 📊 API Reference

### TradingBot Class

#### Methods

- `start()`: Start the trading bot
- `stop()`: Stop the trading bot
- `getState()`: Get current bot state
- `getConfig()`: Get current configuration
- `updateConfig(config)`: Update trading configuration

#### State Object

```typescript
interface BotState {
  isRunning: boolean;
  activeTrades: Map<string, TokenPosition>;
  totalPnl: number;
  totalTrades: number;
  successfulTrades: number;
  failedTrades: number;
}
```

### PumpPortalService Class

#### Methods

- `connect()`: Connect to PumpPortal WebSocket
- `disconnect()`: Disconnect from WebSocket
- `isWebSocketConnected()`: Check connection status

#### Events

- `trade`: Emitted when a new trade is detected
- `error`: Emitted when an error occurs

---

## 🔧 Development

### Project Structure

```
src/
├── constants/          # Configuration constants
├── services/           # External service integrations
│   ├── pumpportal.ts  # PumpPortal WebSocket client
│   ├── telegram.ts    # Telegram notification service
│   └── tokenService.ts # Token metadata service
├── trading/            # Core trading logic
│   └── tradingBot.ts  # Main trading bot class
├── types/              # TypeScript type definitions
├── utils/              # Utility functions
│   ├── helpers.ts     # Helper functions
│   └── logger.ts      # Logging utility
└── index.ts           # Main entry point
```

### Building

```bash
# Build TypeScript
npm run build

# Watch mode
npm run watch

# Clean build
npm run clean
```

### Testing

```bash
# Run tests (when implemented)
npm test

# Run with coverage
npm run test:coverage
```

---

## 🛡️ Security Considerations

### Wallet Security

- **Never commit private keys** to version control
- **Use environment variables** for sensitive data
- **Consider using a hardware wallet** for large amounts
- **Regularly rotate keys** and monitor transactions

### Trading Risks

- **Start with small amounts** to test the bot
- **Monitor performance** regularly
- **Set appropriate stop-losses** to limit downside
- **Understand the risks** of automated trading

### Best Practices

- **Test on devnet** before mainnet
- **Monitor logs** for errors and anomalies
- **Keep the bot updated** with latest changes
- **Backup your configuration** regularly

---

## 📈 Performance Tips

### Optimization

1. **Use a fast RPC endpoint** for better performance
2. **Monitor memory usage** for long-running instances
3. **Adjust trade frequency** based on market conditions
4. **Use appropriate slippage settings** for your strategy

### Monitoring

1. **Set up alerts** for critical errors
2. **Monitor PnL** regularly
3. **Check transaction success rates**
4. **Review trade logs** for patterns

---

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### How to Contribute

1. **Fork the repository**
2. **Create a feature branch**
3. **Make your changes**
4. **Add tests** (if applicable)
5. **Submit a pull request**

### Development Setup

```bash
# Fork and clone
git clone https://github.com/radioman/Auto-solana-trading-bot.git
cd Auto-solana-trading-bot

# Install dependencies
npm install

# Create feature branch
git checkout -b feature/your-feature-name

# Make changes and test
npm run dev

# Submit PR
git push origin feature/your-feature-name
```

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## ⚠️ Disclaimer

**This software is for educational purposes only. Trading cryptocurrencies involves substantial risk of loss and is not suitable for all investors. The high degree of leverage can work against you as well as for you. Before deciding to trade cryptocurrencies, you should carefully consider your investment objectives, level of experience, and risk appetite. The possibility exists that you could sustain a loss of some or all of your initial investment and therefore you should not invest money that you cannot afford to lose. You should be aware of all the risks associated with cryptocurrency trading and seek advice from an independent financial advisor if you have any doubts.**

---

## 🆘 Support

### Getting Help

- **Issues**: [GitHub Issues](https://github.com/radioman/Auto-solana-trading-bot/issues)
- **Discussions**: [GitHub Discussions](https://github.com/radioman/Auto-solana-trading-bot/discussions)
- **Documentation**: [Wiki](https://github.com/radioman/Auto-solana-trading-bot/wiki)

### Common Issues

1. **Connection Issues**: Check your RPC endpoint and internet connection
2. **Transaction Failures**: Verify wallet balance and gas settings
3. **WebSocket Disconnections**: Check network stability and reconnection settings

---

<div align="center">

**Made with ❤️ for the Solana community**

[⭐ Star this repo](https://github.com/radioman/Auto-solana-trading-bot) • [🐛 Report Bug](https://github.com/radioman/Auto-solana-trading-bot/issues) • [💡 Request Feature](https://github.com/radioman/Auto-solana-trading-bot/issues)

</div>
