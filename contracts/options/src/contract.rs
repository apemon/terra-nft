#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, CanonicalAddr, SubMsg, WasmMsg, ReplyOn, Reply, StdError};
use protobuf::Message;

use crate::error::{ContractError};
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, TokenInstantiateMsg};
use crate::state::{Config, CONFIG};
use crate::response::MsgInstantiateContractResponse;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner_addr: deps.api.addr_canonicalize(&info.sender.as_str())?,
        token_addr: CanonicalAddr::from(vec![])
    };
    CONFIG.save(deps.storage, &config)?;

    let message = SubMsg {
        id: 1,
        msg: WasmMsg::Instantiate {
            admin: None,
            code_id: msg.token_code_id,
            msg: to_binary(&TokenInstantiateMsg {
                name: "Option NFT".to_string(),
                symbol:"OPT".to_string(),
                minter: info.sender.to_string()
            })?,
            funds: vec![],
            label: "Option NFT".to_string()
        }.into(),
        gas_limit: None,
        reply_on: ReplyOn::Success
    };
    Ok(Response::new()
        .add_submessage(message)
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
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    let data = msg.result.unwrap().data.unwrap();
    let res: MsgInstantiateContractResponse =
        Message::parse_from_bytes(data.as_slice()).map_err(|_| {
            StdError::parse_err("MsgInstantiateContractResponse", "failed to parse data")
        })?;
    let token_addr = res.get_contract_address();

    let api = deps.api;
    CONFIG.update(deps.storage, |mut meta| -> StdResult<_> {
        meta.token_addr = api.addr_canonicalize(token_addr)?;
        Ok(meta)
    })?;

    Ok(Response::new().add_attribute("nft_token_addr", token_addr))
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