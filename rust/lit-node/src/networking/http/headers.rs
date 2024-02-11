pub const NODE_REQUEST_BODY_DESCRIPTOR_KEY: &str = "node-request-body-descriptor";

pub const DESCRIPTOR_VALUE_BODY_SEPARATOR: &str = "; ";

pub fn join_multiple_body_descriptor_parameters<M>(body_data: &[M]) -> String
where
    M: AsRef<str>,
{
    body_data
        .iter()
        .map(|body_data| body_data.as_ref())
        .collect::<Vec<&str>>()
        .join(DESCRIPTOR_VALUE_BODY_SEPARATOR)
}
