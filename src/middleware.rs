use actix_governor::{
    GovernorConfig, GovernorConfigBuilder, 
    PeerIpKeyExtractor, governor::middleware::StateInformationMiddleware
};

pub fn shorten_governor() -> GovernorConfig<PeerIpKeyExtractor, StateInformationMiddleware> {
    GovernorConfigBuilder::default()
        .per_second(3)
        .burst_size(20)
        .use_headers()
        .finish()
        .unwrap()
}

pub fn redirect_governor() -> GovernorConfig<PeerIpKeyExtractor, StateInformationMiddleware> {
    GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(30)
        .use_headers()
        .finish()
        .unwrap()
}
