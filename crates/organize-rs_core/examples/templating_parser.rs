use winnow::{
    ascii::alpha0,
    combinator::alt,
    error::{Error, ErrorKind},
    multi::separated0,
    sequence::{delimited, preceded},
    token::take_while,
    IResult, Parser,
};
use winnow::{
    stream::AsChar,
    token::{any, tag},
};

pub fn parse_text_incl_underscore_hyphen(input: &str) -> IResult<&str, &str> {
    take_while(1.., (AsChar::is_alpha, ('_'), ('-'))).parse_next(input)
}

pub fn parse_boundary(input: &str) -> IResult<&str, &str> {
    take_while(1.., "{()}").parse_next(input)
}

pub fn parse_single_template(input: &str) -> IResult<&str, &str> {
    let (remainder, (_, text, _)) =
        (parse_boundary, parse_template_content, parse_boundary).parse_next(input)?;
    Ok((remainder, text))
}

pub fn parse_transform_template(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (_, template) = parse_single_template.parse_next(input)?;

    let (remainder, (text, _, arg, _)) = (
        parse_text_incl_underscore_hyphen,
        parse_boundary,
        parse_separated_text,
        parse_boundary,
    )
        .parse_next(template)?;
    Ok((remainder, (text, arg)))
}

pub fn parse_dotted_template(input: &str) -> IResult<&str, Vec<&str>> {
    let (_, template) = parse_single_template.parse_next(input)?;

    (parse_separated_text).parse_next(template)
}

fn parse_separated_text(input: &str) -> IResult<&str, Vec<&str>> {
    // take_while(1.., (AsChar::is_alpha, ('.'))).parse_next(input)
    separated0(parse_text_incl_underscore_hyphen, '.').parse_next(input)
}

fn parse_template_content(input: &str) -> IResult<&str, &str> {
    take_while(1.., (AsChar::is_alpha, ('.'), ('_'), ('('), (')'))).parse_next(input)
}

// pub fn parse_transform_template_with_sub_key(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
//     let (remainder, (function, arg)) = parse_transform_template.parse_next(input)?;
//     let (_, args) = parse_arg_dot.parse_next(arg)?;
//     Ok((remainder, (function, args)))
// }

fn main() {
    let case1 = "{{extension}}";
    let case2 = "{{uppercase(extension)}}";
    let case3 = "{{lowercase(entry.name)}}";
    let case4 = "{{entry.created.year}}";
    let case5 = "{{file_content.customer}}";
    let case6 = "{{entry.metadata.last_modified.year}}";
    let case7 = "{{size.traditional}} -- {{relative_path}}";
    let case8 = "{{date_added.strftime('%Y-%m-%d')}}";

    let (remainder, result) = (parse_single_template).parse_next(case1).unwrap();
    assert_eq!(result, ("extension"));
    let (remainder, result) = (parse_transform_template).parse_next(case2).unwrap();
    assert_eq!(result, ("uppercase", vec!["extension"]));
    let (remainder, result) = (parse_transform_template).parse_next(case3).unwrap();
    assert_eq!(result, ("lowercase", vec!["entry", "name"]));
    let (remainder, result) = (parse_dotted_template).parse_next(case4).unwrap();
    assert_eq!(result, (vec!["entry", "created", "year"]));
    let (remainder, result) = (parse_dotted_template).parse_next(case5).unwrap();
    assert_eq!(result, (vec!["file_content", "customer"]));
    let (remainder, result) = (parse_dotted_template).parse_next(case6).unwrap();
    assert_eq!(result, (vec!["entry", "metadata", "last_modified", "year"]));

    println!("{remainder} - {result:?}");
}
