pub fn validate_msg(log_msg: String) -> (String, usize) {
    let mut new_msg = String::new();

    // "{{{ab}{cw}}}"
    // "{}}{{}"

    for (idx, chr) in log_msg.chars().enumerate() {
        //
    }

    todo!()
}

// =========================================== //
// =========================================== //
// =========================================== //
// =========================================== //

pub fn destructure_msg(log_msg: String) -> Result<String, String> {
    let mut args: Vec<Option<String>> = vec![];

    let mut skip_count = 0_usize;
    let mut arg_name = String::new();

    let mut arg_name_enter_mode = false;
    let mut bracket_left_mode = false;
    let mut bracket_right_mode = false;

    for (idx, chr) in log_msg.chars().enumerate() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        match arg_name_enter_mode {
            false => match chr {
                '{' => {
                    arg_name_enter_mode = true;
                    bracket_left_mode = true;
                }
                '}' => bracket_right_mode = true,
                _ => (),
            },
            true => (),
        }
    }

    Ok(log_msg)
}
