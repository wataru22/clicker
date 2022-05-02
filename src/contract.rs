#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
// We're adding to_binary, Binary, Deps and StdResult 
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
// We're adding InstantiateMsg and QueryMsg
use crate::msg::{LuckyNumberResponse, LuckyStringResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:clicker";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  // Ok(Response::default())
  
  // We're storing stuff in a variable called "state" of type "State"
  let state = State {
    lucky_number: msg.lucky_number,
    // lucky_string: msg.lucky_string,
    owner: info.sender.clone(),
  };

  // We're setting the contract version using a helper function we imported
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  // We're storing state in a special variable called "STATE"
  STATE.save(deps.storage, &state)?;

  // Sending a response back to the caller
  Ok(Response::new()
    .add_attribute("method", "instantiate")
    .add_attribute("owner", info.sender)
    .add_attribute("lucky_number", msg.lucky_number.to_string())
    // .add_attribute("lucky_string", msg.lucky_string.to_string())
  )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
      QueryMsg::GetLuckyNumber {} => to_binary(&query_lucky_number(deps)?),
      // QueryMsg::GetLuckyString {} => to_binary(&query_lucky_string(deps)?),
  }
}

fn query_lucky_number(deps: Deps) -> StdResult<LuckyNumberResponse> {
  let state = STATE.load(deps.storage)?;
  Ok(LuckyNumberResponse { lucky_number: state.lucky_number })
}

// fn query_lucky_string(deps: Deps) -> StdResult<LuckyStringResponse> {
//   let state = STATE.load(deps.storage)?;
//   Ok(LuckyStringResponse { lucky_string: state.lucky_string })
// }