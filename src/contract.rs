#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:coins_order_test";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg { ExecuteMsg::SendCoins { sort } => {
        let mut balances = deps.querier.query_all_balances(env.contract.address).unwrap();

        if sort {
            balances.sort_by(|a, b| a.denom.cmp(&b.denom));
        } else {
            balances.sort_by(|a, b| b.denom.cmp(&a.denom));
        }

        let msg = BankMsg::Send { to_address: info.sender.to_string(), amount: balances.clone() };

        Ok(Response::new()
            .add_message(msg)
            .add_attribute("action", "send_coins")
            .add_attribute("sending order", balances.iter().map(|x| x.denom.clone()).collect::<Vec<String>>().join(", "))
        )
    } }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
