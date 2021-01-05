use nom::branch::alt;
use nom::{named, tag_no_case, take_while1, IResult};

#[derive(Debug, PartialEq)]
pub enum ParserResult {
    CapabilityRequest(String),
    AuthPlainRequest(String),
    Unknown,
}

named!(cap_command ( &str ) -> &str, tag_no_case!("capability"));
named!(auth_command ( &str ) -> &str, tag_no_case!("authenticate plain"));
named!(imaptag ( &str ) -> &str, take_while1!(|c: char| c.is_alphanumeric()));
named!(space ( &str ) -> &str, take_while1!(|c| c == ' '));

fn capability(input: &str) -> IResult<&str, ParserResult> {
    let (input, imap_tag) = imaptag(input)?;
    let (input, _) = space(input)?;
    let (input, _) = cap_command(input)?;
    Ok((input, ParserResult::CapabilityRequest(imap_tag.to_owned())))
}

fn auth_plain(input: &str) -> IResult<&str, ParserResult> {
    let (input, imap_tag) = imaptag(input)?;
    let (input, _) = space(input)?;
    let (input, _) = auth_command(input)?;
    Ok((input, ParserResult::CapabilityRequest(imap_tag.to_owned())))
}

pub fn commands(input: &str) -> IResult<&str, ParserResult> {
    let (input, result) = alt((capability, auth_plain))(input)?;
    return Ok((input, result));
}
