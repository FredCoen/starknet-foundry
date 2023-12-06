use addition_virtual::add;

fn fib(a: felt252, b: felt252, n: felt252) -> felt252 {
    match n {
        0 => a,
        _ => fib(b, add(a, b), n - 1),
    }
}

#[starknet::contract]
mod FibonacciContract {
    use addition_virtual::add;
    use fibonacci::fib;

    #[storage]
    struct Storage {}

    #[external(v0)]
    fn answer(ref self: ContractState) -> felt252 {
        add(fib(0, 1, 16), fib(0, 1, 8))
    }
}

#[cfg(test)]
mod tests {
    use super::fib;
    use snforge_std::declare;

    #[test]
    fn it_works() {
        assert(fib(0, 1, 16) == 987, 'it works!');
    }

    #[test]
    fn contract_test() {
        declare('FibonacciContract');
        declare('AdditionContract');
    }
}