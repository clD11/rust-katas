fn schedule_job(f: fn(u32) -> u32, arg: u32) -> u32 {
    return f(arg)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_schedule_jobs() {
        let f = |i: u32| -> u32 { i + i };
        let actual = schedule_job(f,1);
        assert_eq!(actual, 2)
    }

}