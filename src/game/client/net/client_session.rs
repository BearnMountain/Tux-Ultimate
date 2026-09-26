enum ConnectionState {
    Idle,
    Resolving,
    Handshaking,
    Authenticating,
    Joined,
    // Failed(ConnectionError),
    Cancelled,
}
