use hyperliquid_rust_sdk::messages::{
    ApproveAgentRequest, ApproveBuilderFeeRequest, SetReferrerRequest, UpdateIsolatedMarginRequest,
    CancelOrderRequest, ModifyOrderRequest, OrderRequest, UpdateLeverageRequest,
    ClassTransferRequest, TransferRequest, WithdrawRequest,
    ExchangeMessage, MessageType, MessageHeader
};

#[test]
fn test_order_request_serialization() {
    // Test market order
    let market_order = OrderRequest::market("BTC", true, "1.0");
    let serialized = market_order.to_msgpack().unwrap();
    let deserialized = OrderRequest::from_msgpack(&serialized).unwrap();
    assert_eq!(market_order.asset, deserialized.asset);
    assert_eq!(market_order.size, deserialized.size);
    assert_eq!(market_order.limit_price, deserialized.limit_price);

    // Test limit order
    let limit_order = OrderRequest::limit("ETH", false, "2.0", "1800.0");
    let serialized = limit_order.to_msgpack().unwrap();
    let deserialized = OrderRequest::from_msgpack(&serialized).unwrap();
    assert_eq!(limit_order.asset, deserialized.asset);
    assert_eq!(limit_order.size, deserialized.size);
    assert_eq!(limit_order.limit_price, deserialized.limit_price);
}

#[test]
fn test_cancel_order_request_serialization() {
    // Test cancel by order ID
    let cancel_by_id = CancelOrderRequest::by_order_id("BTC", 12345);
    let serialized = cancel_by_id.to_msgpack().unwrap();
    let deserialized = CancelOrderRequest::from_msgpack(&serialized).unwrap();
    assert_eq!(cancel_by_id.asset, deserialized.asset);
    assert_eq!(cancel_by_id.order_id, deserialized.order_id);
    assert_eq!(cancel_by_id.cloid, deserialized.cloid);

    // Test cancel by client order ID
    let cancel_by_cloid = CancelOrderRequest::by_cloid("BTC", "client-123");
    let serialized = cancel_by_cloid.to_msgpack().unwrap();
    let deserialized = CancelOrderRequest::from_msgpack(&serialized).unwrap();
    assert_eq!(cancel_by_cloid.asset, deserialized.asset);
    assert_eq!(cancel_by_cloid.order_id, deserialized.order_id);
    assert_eq!(cancel_by_cloid.cloid, deserialized.cloid);
}

#[test]
fn test_transfer_request_serialization() {
    let transfer = TransferRequest::new("USDC", "100.0", "0x1234...");
    let serialized = transfer.to_msgpack().unwrap();
    let deserialized = TransferRequest::from_msgpack(&serialized).unwrap();
    assert_eq!(transfer.asset, deserialized.asset);
    assert_eq!(transfer.amount, deserialized.amount);
    assert_eq!(transfer.destination, deserialized.destination);
}

#[test]
fn test_withdraw_request_serialization() {
    let withdraw = WithdrawRequest::new("USDC", "50.0", "0x5678...");
    let serialized = withdraw.to_msgpack().unwrap();
    let deserialized = WithdrawRequest::from_msgpack(&serialized).unwrap();
    assert_eq!(withdraw.asset, deserialized.asset);
    assert_eq!(withdraw.amount, deserialized.amount);
    assert_eq!(withdraw.destination, deserialized.destination);
}

#[test]
fn test_class_transfer_request_serialization() {
    let class_transfer = ClassTransferRequest::new(1000.0, true);
    let serialized = class_transfer.to_msgpack().unwrap();
    let deserialized = ClassTransferRequest::from_msgpack(&serialized).unwrap();
    assert_eq!(class_transfer.amount, deserialized.amount);
    assert_eq!(class_transfer.to_perp, deserialized.to_perp);
}

#[test]
fn test_update_isolated_margin_request_serialization() {
    let update_margin = UpdateIsolatedMarginRequest::new("BTC", 1000.0);
    let serialized = update_margin.to_msgpack().unwrap();
    let deserialized = UpdateIsolatedMarginRequest::from_msgpack(&serialized).unwrap();
    assert_eq!(update_margin.asset, deserialized.asset);
    assert_eq!(update_margin.amount, deserialized.amount);
}

#[test]
fn test_approve_agent_request_serialization() {
    let approve_agent = ApproveAgentRequest::new("0x1234...");
    let serialized = approve_agent.to_msgpack().unwrap();
    let deserialized = ApproveAgentRequest::from_msgpack(&serialized).unwrap();
    assert_eq!(approve_agent.agent_address, deserialized.agent_address);
    assert_eq!(approve_agent.agent_name, deserialized.agent_name);
}

#[test]
fn test_set_referrer_request_serialization() {
    let set_referrer = SetReferrerRequest::new("REF123");
    let serialized = set_referrer.to_msgpack().unwrap();
    let deserialized = SetReferrerRequest::from_msgpack(&serialized).unwrap();
    assert_eq!(set_referrer.code, deserialized.code);
}

#[test]
fn test_approve_builder_fee_request_serialization() {
    let approve_builder_fee = ApproveBuilderFeeRequest::new("0x1234...", "0.001");
    let serialized = approve_builder_fee.to_msgpack().unwrap();
    let deserialized = ApproveBuilderFeeRequest::from_msgpack(&serialized).unwrap();
    assert_eq!(approve_builder_fee.builder, deserialized.builder);
    assert_eq!(approve_builder_fee.max_fee_rate, deserialized.max_fee_rate);
}

#[test]
fn test_message_type_values() {
    // Verify that message type values are as expected
    assert_eq!(MessageType::Order as u8, 0x01);
    assert_eq!(MessageType::CancelOrder as u8, 0x02);
    assert_eq!(MessageType::ModifyOrder as u8, 0x03);
    assert_eq!(MessageType::UpdateLeverage as u8, 0x04);
    assert_eq!(MessageType::Transfer as u8, 0x10);
    assert_eq!(MessageType::Withdraw as u8, 0x11);
    assert_eq!(MessageType::ClassTransfer as u8, 0x12);
    assert_eq!(MessageType::UpdateIsolatedMargin as u8, 0x20);
    assert_eq!(MessageType::ApproveAgent as u8, 0x21);
    assert_eq!(MessageType::SetReferrer as u8, 0x22);
    assert_eq!(MessageType::ApproveBuilderFee as u8, 0x23);
}
