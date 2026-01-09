# Platform Logos

This directory contains logos for trading platforms used in indicator pages.

## File Naming Convention

```
platform-name.png          → Standard resolution (200-300px wide)
platform-name@2x.png       → Retina/high-res (400-600px wide)
platform-name-dark.png     → Dark mode variant (optional)
```

## Supported Platforms

Add logos for:
- `thinkorswim.png` - ThinkorSwim platform
- `tradingview.png` - TradingView platform
- `ninjatrader.png` - NinjaTrader platform
- `metatrader.png` - MetaTrader platform
- `tradestation.png` - TradeStation platform

## Usage in Code

```svelte
<img 
    src="/logos/platforms/thinkorswim.png"
    srcset="/logos/platforms/thinkorswim@2x.png 2x"
    alt="ThinkorSwim"
    width="200"
/>
```

## Image Specifications

- **Format**: PNG with transparent background
- **Standard size**: 200-300px wide
- **Retina size**: 400-600px wide (2x)
- **File size**: Keep under 50KB per file
- **Optimization**: Use tools like TinyPNG or ImageOptim
