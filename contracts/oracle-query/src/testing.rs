use std::collections::HashMap;

use anyhow::{bail, Ok};
use cosmwasm_std::{
    Addr, AnyMsg, Api, Binary, BlockInfo, CustomMsg, CustomQuery, Deps, GrpcQuery, Querier, Storage,
};
use cw_multi_test::error::AnyResult;
use cw_multi_test::{no_init, AppBuilder, AppResponse, CosmosRouter, Stargate};
use nibiru_std::proto::nibiru::oracle::{
    ExchangeRateTuple, QueryExchangeRateResponse, QueryExchangeRatesResponse,
};
use prost::Message;
use serde::de::DeserializeOwned;

use crate::contract::query::{query_exchange_rate, query_exchange_rate_twap, query_exchange_rates};
use crate::msg::{GetExchangeRateResponse, GetExchangeRatesResponse};

const MSG_STARGATE_EXECUTE: &str = "stargate execute called";
const MSG_STARGATE_QUERY: &str = "stargate query called";
const MSG_ANY_EXECUTE: &str = "any execute called";

struct StargateKeeper;

impl Stargate for StargateKeeper {
    fn execute_stargate<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _sender: Addr,
        _type_url: String,
        _value: Binary,
    ) -> AnyResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        bail!(MSG_STARGATE_EXECUTE)
    }

    fn query_stargate(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        _path: String,
        _data: Binary,
    ) -> AnyResult<Binary> {
        bail!(MSG_STARGATE_QUERY)
    }

    fn execute_any<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _sender: Addr,
        _msg: AnyMsg,
    ) -> AnyResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        bail!(MSG_ANY_EXECUTE)
    }

    fn query_grpc(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        request: GrpcQuery,
    ) -> AnyResult<Binary> {
        match request.path.as_str() {
            "/nibiru.oracle.v1.Query/ExchangeRate" => Ok(Binary::from(
                QueryExchangeRateResponse {
                    exchange_rate: "1.0".to_string(),
                }
                .encode_to_vec(),
            )),
            "/nibiru.oracle.v1.Query/ExchangeRateTwap" => Ok(Binary::from(
                QueryExchangeRateResponse {
                    exchange_rate: "2.0".to_string(),
                }
                .encode_to_vec(),
            )),
            "/nibiru.oracle.v1.Query/ExchangeRates" => Ok(Binary::from(
                QueryExchangeRatesResponse {
                    exchange_rates: vec![
                        ExchangeRateTuple {
                            pair: "ubtc:uusd".to_string(),
                            exchange_rate: "3.0".to_string(),
                        },
                        ExchangeRateTuple {
                            pair: "ueth:uusd".to_string(),
                            exchange_rate: "4.0".to_string(),
                        },
                    ],
                }
                .encode_to_vec(),
            )),
            _ => bail!("unexpected grpc query"),
        }
    }
}

#[test]
fn test_query_exchange_rate() {
    // build the application with custom stargate keeper
    let app = AppBuilder::default()
        .with_stargate(StargateKeeper)
        .build(no_init);

    let deps = Deps {
        api: app.api(),
        storage: app.storage(),
        querier: app.wrap(),
    };
    let resp = query_exchange_rate(deps, "ubtc:uusd".to_string()).unwrap();

    assert_eq!(
        resp,
        GetExchangeRateResponse {
            price: "1.0".to_string()
        }
    )
}

#[test]
fn test_query_exchange_rate_twap() {
    // build the application with custom stargate keeper
    let app = AppBuilder::default()
        .with_stargate(StargateKeeper)
        .build(no_init);

    let deps = Deps {
        api: app.api(),
        storage: app.storage(),
        querier: app.wrap(),
    };
    let resp = query_exchange_rate_twap(deps, "ubtc:uusd".to_string()).unwrap();

    assert_eq!(
        resp,
        GetExchangeRateResponse {
            price: "2.0".to_string()
        }
    )
}

#[test]
fn test_query_exchange_rates() {
    // build the application with custom stargate keeper
    let app = AppBuilder::default()
        .with_stargate(StargateKeeper)
        .build(no_init);

    let deps = Deps {
        api: app.api(),
        storage: app.storage(),
        querier: app.wrap(),
    };
    let resp = query_exchange_rates(deps).unwrap();

    assert_eq!(
        resp,
        GetExchangeRatesResponse {
            rates: HashMap::from_iter(vec![
                ("ubtc:uusd".to_string(), "3.0".to_string()),
                ("ueth:uusd".to_string(), "4.0".to_string()),
            ])
        }
    )
}
