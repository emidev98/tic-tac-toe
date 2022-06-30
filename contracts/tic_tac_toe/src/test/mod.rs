#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, StdError};

    use crate::contract::instantiate::instantiate;
    use crate::contract::query::query;
    use crate::models::{responses::MatchResponse, InstantiateMsg, QueryMsg};

    #[test]
    fn query_empty_matches_by_host() {
        // GIVEN
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("host", &coins(2, "token"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // WHEN
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Games {
                host: Some(String::from("host")),
                opponent: None,
            },
        );

        // THEN
        let value: MatchResponse = from_binary(&res.unwrap()).unwrap();
        assert_eq!(
            value,
            MatchResponse {
                host: Some(String::from("host")),
                opponent: None,
                matches: vec![]
            }
        );
    }

    #[test]
    fn query_empty_matches_by_opponent() {
        // GIVEN
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("host", &coins(2, "token"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // WHEN
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Games {
                host: None,
                opponent: Some(String::from("opponent")),
            },
        );

        // THEN
        let value: MatchResponse = from_binary(&res.unwrap()).unwrap();
        assert_eq!(
            value,
            MatchResponse {
                host: None,
                opponent: Some(String::from("opponent")),
                matches: vec![]
            }
        );
    }

    #[test]
    fn query_empty_matches_with_both_users() {
        // GIVEN
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("host", &coins(2, "token"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // WHEN
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Games {
                host: Some(String::from("host")),
                opponent: Some(String::from("opponent")),
            },
        );

        // THEN
        let value: MatchResponse = from_binary(&res.unwrap()).unwrap();
        assert_eq!(
            value,
            MatchResponse {
                host: Some(String::from("host")),
                opponent: Some(String::from("opponent")),
                matches: vec![]
            }
        );
    }

    #[test]
    fn query_empty_matches_with_no_users() {
        // GIVEN
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("host", &coins(2, "token"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // WHEN
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Games {
                host: None,
                opponent: None,
            },
        );

        // THEN
        let value: MatchResponse = from_binary(&res.unwrap()).unwrap();
        assert_eq!(
            value,
            MatchResponse {
                host: None,
                opponent: None,
                matches: vec![]
            }
        );
    }

    #[test]
    fn query_host_error() {
        // GIVEN
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("host", &coins(2, "token"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // WHEN
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Games {
                host: Some(String::from("w")),
                opponent: None,
            },
        );

        // THEN
        assert_eq!(
            res.unwrap_err(),
            StdError::GenericErr {
                msg: String::from("Invalid input: human address too short")
            }
        );
    }

    #[test]
    fn query_opponent_error() {
        // GIVEN
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("host", &coins(2, "token"));
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // WHEN
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Games {
                host: Some(String::from("host")),
                opponent: Some(String::from("w")),
            },
        );

        // THEN
        assert_eq!(
            res.unwrap_err(),
            StdError::GenericErr {
                msg: String::from("Invalid input: human address too short")
            }
        );
    }
}