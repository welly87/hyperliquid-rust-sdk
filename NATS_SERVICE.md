# HyperLiquid NATS Service

This service listens for NATS messages and processes them as HyperLiquid orders.

## Prerequisites

- Docker and Docker Compose
- Rust (if building locally)

## Running with Docker Compose

The easiest way to run the service is using Docker Compose:

```bash
docker-compose up --build
```

This will start:
1. A NATS server with JetStream enabled
2. The HyperLiquid NATS service

## Configuration

You can configure the service using environment variables:

- `NATS_URL`: NATS server URL (default: `nats://localhost:4222`)
- `NATS_SUBJECT`: NATS subject to subscribe to (default: `hyperliquid.orders`)
- `HYPERLIQUID_API_URL`: HyperLiquid API URL (default: `https://api.hyperliquid.xyz`)
- `RUST_LOG`: Log level (default: `info`)

## Sending Orders

Publish a JSON message to the configured NATS subject. The message should include a header with message metadata:

```json
{
  "header": {
    "message_type": "order_request",
    "message_id": "550e8400-e29b-41d4-a716-446655440000",
    "correlation_id": "some-correlation-id-123",
    "timestamp": 1620000000000
  },
  "action": "market_order",
  "coin": "BTC",
  "is_buy": true,
  "sz": "0.01",
  "cloid": "unique-order-id-123"
}

// or for limit orders
{
  "header": {
    "message_type": "limit_order_request",
    "message_id": "550e8400-e29b-41d4-a716-446655440001",
    "correlation_id": "some-correlation-id-456",
    "timestamp": 1620000000001
  },
  "action": "limit_order",
  "coin": "ETH",
  "is_buy": false,
  "sz": "1.0",
  "limit_px": "1800.50",
  "cloid": "unique-order-id-456"
}

// You can also use NATS headers for message metadata
nats pub hyperliquid.orders \
  -H "message_type:order_request" \
  -H "correlation_id:some-correlation-id-789" \
  '{"action":"market_order","coin":"BTC","is_buy":true,"sz":"0.01"}'
```

## Building Locally

1. Install Rust: https://rustup.rs/
2. Build the project:
   ```bash
   cargo build --release --bin nats_service
   ```
3. Run the service:
   ```bash
   RUST_LOG=info ./target/release/nats_service
   ```

## Testing

You can use the `nats` CLI to test the service:

```bash
# Publish a test message
nats pub hyperliquid.orders '{"action":"market_order","coin":"BTC","is_buy":true,"sz":"0.01"}'
```

## Adding New Message Types

Message handlers are registered in a global `HashMap` inside
`src/bin/nats_service.rs`. To support a new message:

1. Create a new struct implementing `ExchangeMessage` in `src/messages`.
2. Add an async handler function that deserializes the message and calls the
   appropriate `ExchangeClient` method.
3. Insert the handler into the `HANDLERS` map with the corresponding
   `MessageType`.

Once added, any message of that type published to the configured NATS subject
will be routed to your handler automatically.
