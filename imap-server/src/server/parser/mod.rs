use nom::branch::alt;
use nom::sequence::tuple;
use nom::{delimited, named, tag, tag_no_case, take_while, take_while1, IResult};

#[derive(Debug, PartialEq)]
pub enum ParserResult {
    CapabilityRequest(String),
    AuthPlainContinueRequest(String),
    LoginRequest(String, String, String),
    CloseRequest(String),
    LogoutRequest(String),
    ListRequest(String, String, String),
    LSUBRequest(String, String, String),
    CreateRequest(String, String),
    SubscribeRequest(String, String),
    SelectRequest(String, String),
}

named!(cap_command ( &str ) -> &str, tag_no_case!("capability"));
named!(auth_command ( &str ) -> &str, tag_no_case!("authenticate plain"));
named!(login_command ( &str ) -> &str, tag_no_case!("login"));
named!(list_command ( &str ) -> &str, tag_no_case!("list"));
named!(lsub_command ( &str ) -> &str, tag_no_case!("lsub"));
named!(create_command ( &str ) -> &str, tag_no_case!("create"));
named!(subscribe_command ( &str ) -> &str, tag_no_case!("subscribe"));
named!(select_command ( &str ) -> &str, tag_no_case!("select"));
named!(close_command ( &str ) -> &str, tag_no_case!("close"));
named!(logout_command ( &str ) -> &str, tag_no_case!("logout"));
named!(imaptag ( &str ) -> &str, take_while1!(|c: char| c.is_alphanumeric()));
named!(space ( &str ) -> &str, take_while1!(|c: char| c.is_whitespace()));

named!(string_inner ( &str ) -> &str, take_while!(|c: char| c.is_alphanumeric() && c != '"' && c != ' ' || c == '*'));
named!(email ( &str ) -> &str, take_while!(|c: char| c.is_alphanumeric() && c != '"' && c != ' ' || c == '@' || c == '_' || c == '-'));
named!(basic_string ( &str ) -> &str, delimited!(
    tag!("\""),
    string_inner,
    tag!("\"")
));
named!(email_string ( &str ) -> &str, delimited!(
    tag!("\""),
    email,
    tag!("\"")
));

fn capability(input: &str) -> IResult<&str, ParserResult> {
    let (input, (imap_tag, _, _)) = tuple((imaptag, space, cap_command))(input)?;
    Ok((input, ParserResult::CapabilityRequest(imap_tag.to_string())))
}

fn close(input: &str) -> IResult<&str, ParserResult> {
    let (input, (imap_tag, _, _)) = tuple((imaptag, space, close_command))(input)?;
    Ok((input, ParserResult::CloseRequest(imap_tag.to_string())))
}

fn logout(input: &str) -> IResult<&str, ParserResult> {
    let (input, (imap_tag, _, _)) = tuple((imaptag, space, logout_command))(input)?;
    Ok((input, ParserResult::LogoutRequest(imap_tag.to_string())))
}

fn auth_plain_continue(input: &str) -> IResult<&str, ParserResult> {
    let (input, (imap_tag, _, _)) = tuple((imaptag, space, auth_command))(input)?;
    Ok((
        input,
        ParserResult::AuthPlainContinueRequest(imap_tag.to_string()),
    ))
}

fn login(input: &str) -> IResult<&str, ParserResult> {
    let (input, (imap_tag, _, _, _, username, _, password)) = tuple((
        imaptag,
        space,
        login_command,
        space,
        email_string,
        space,
        basic_string,
    ))(input)?;
    Ok((
        input,
        ParserResult::LoginRequest(
            imap_tag.to_string(),
            username.to_string(),
            password.to_string(),
        ),
    ))
}

fn list(input: &str) -> IResult<&str, ParserResult> {
    let (input, (imap_tag, _, _, _, mailbox, _, interpretation)) = tuple((
        imaptag,
        space,
        list_command,
        space,
        basic_string,
        space,
        basic_string,
    ))(input)?;
    Ok((
        input,
        ParserResult::ListRequest(
            imap_tag.to_string(),
            mailbox.to_string(),
            interpretation.to_string(),
        ),
    ))
}

fn lsub(input: &str) -> IResult<&str, ParserResult> {
    let (input, (imap_tag, _, _, _, mailbox, _, interpretation)) = tuple((
        imaptag,
        space,
        lsub_command,
        space,
        basic_string,
        space,
        basic_string,
    ))(input)?;
    Ok((
        input,
        ParserResult::LSUBRequest(
            imap_tag.to_string(),
            mailbox.to_string(),
            interpretation.to_string(),
        ),
    ))
}

fn create(input: &str) -> IResult<&str, ParserResult> {
    let (input, (imap_tag, _, _, _, folder)) =
        tuple((imaptag, space, create_command, space, basic_string))(input)?;
    Ok((
        input,
        ParserResult::CreateRequest(imap_tag.to_string(), folder.to_string()),
    ))
}
fn subscribe(input: &str) -> IResult<&str, ParserResult> {
    let (input, (imap_tag, _, _, _, folder)) =
        tuple((imaptag, space, subscribe_command, space, basic_string))(input)?;
    Ok((
        input,
        ParserResult::SubscribeRequest(imap_tag.to_string(), folder.to_string()),
    ))
}
fn select(input: &str) -> IResult<&str, ParserResult> {
    let (input, (imap_tag, _, _, _, folder)) =
        tuple((imaptag, space, select_command, space, basic_string))(input)?;
    Ok((
        input,
        ParserResult::SelectRequest(imap_tag.to_string(), folder.to_string()),
    ))
}

pub fn commands(input: &str) -> IResult<&str, ParserResult> {
    let (input, result) = alt((
        capability,
        auth_plain_continue,
        login,
        list,
        lsub,
        create,
        subscribe,
        select,
        close,
        logout,
    ))(input)?;
    Ok((input, result))
}
