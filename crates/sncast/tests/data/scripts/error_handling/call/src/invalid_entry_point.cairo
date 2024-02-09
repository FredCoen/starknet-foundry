use sncast_std::{
    call, CallResult, ScriptCommandError, RPCError, StarknetError
};


fn main() {
    let eth = 0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7;
    let call_err: ScriptCommandError = call(
        eth.try_into().expect('bad address'), 'gimme_money', array![]
    )
        .unwrap_err();

    println!("{:?}", call_err);

    assert(
        ScriptCommandError::RPCError(
            RPCError::StarknetError(StarknetError::ContractError)
        ) == call_err,
        'ohno'
    )
}