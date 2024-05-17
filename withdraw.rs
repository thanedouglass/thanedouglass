pub fn withdraw(e: Env, to: Address) {
    let state = get_state(&e);
    let recipient = get_recipient(&e);

    match state {
        State::Running => {
            panic!("sale is still running")
        }
        State::Success => {
            assert!(
                to == recipient,
                "sale was successful, only the recipient may withdraw"
            );
            assert!(
                !get_recipient_claimed(&e),
                "sale was successful, recipient has withdrawn funds already"
            );

            let token = get_token(&e);
            transfer(&e, &recipient, &get_balance(&e, &token));
            set_recipient_claimed(&e);
        }
        State::Expired => {
            assert!(
                to != recipient,
                "sale expired, the recipient may not withdraw"
            );

            // Withdraw full amount
            let balance = get_user_deposited(&e, &to);
            set_user_deposited(&e, &to, &0);
            transfer(&e, &to, &balance);
    
        }
    };
}
