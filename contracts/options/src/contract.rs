#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, CanonicalAddr};

use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner_addr: deps.api.addr_canonicalize(&info.sender.as_str())?,
        token_addr: CanonicalAddr::from(vec![])
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { ask:_, offer:_, expire:_ } => {
            Ok(Response::default())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps, env)?),
    }
}

fn query_config(
    deps: Deps,
    env: Env
) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        contract_addr: env.contract.address,
        owner_addr: deps.api.addr_humanize(&config.owner_addr)?,
        token_addr: deps.api.addr_humanize(&config.token_addr)?,
     })
}