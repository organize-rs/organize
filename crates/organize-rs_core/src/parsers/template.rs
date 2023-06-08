use winnow::{
    combinator::{alt},
    multi::separated0,
    sequence::{delimited, separated_pair},
    token::take_while,
    IResult, Parser,
};
use winnow::{
    stream::AsChar,
};

pub fn parse_text_incl_underscore_hyphen(input: &str) -> IResult<&str, &str> {
    take_while(1.., (AsChar::is_alpha, ('_'), ('-'))).parse_next(input)
}

pub fn parse_boundaries(input: &str) -> IResult<&str, &str> {
    take_while(1.., "{()}").parse_next(input)
}

pub fn parse_function_boundary(input: &str) -> IResult<&str, &str> {
    take_while(1.., "()").parse_next(input)
}

pub fn parse_template_outer_boundary(input: &str) -> IResult<&str, &str> {
    take_while(1.., "{}").parse_next(input)
}

pub fn parse_template(input: &str) -> IResult<&str, &str> {
    (delimited(
        parse_template_outer_boundary,
        parse_template_content,
        parse_template_outer_boundary,
    ))
    .parse_next(input)
}

pub fn is_template_boundary(character: char) -> bool {
    matches!(character, '{' | '}')
}

pub fn parse_transform_template(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (_, template) = parse_template.parse_next(input)?;

    (
        parse_text_incl_underscore_hyphen,
        delimited(
            parse_function_boundary,
            parse_dot_separated_text,
            parse_function_boundary,
        ),
    )
        .parse_next(template)
}

pub fn parse_dotted_template(input: &str) -> IResult<&str, Vec<&str>> {
    let (_, template) = parse_template.parse_next(input)?;

    (parse_dot_separated_text).parse_next(template)
}

fn parse_dot_separated_text(input: &str) -> IResult<&str, Vec<&str>> {
    // take_while(1.., (AsChar::is_alpha, ('.'))).parse_next(input)
    separated0(parse_text_incl_underscore_hyphen, '.').parse_next(input)
}

fn parse_template_content(input: &str) -> IResult<&str, &str> {
    take_while(1.., (AsChar::is_alpha, "._()-%, '")).parse_next(input)
}

fn parse_strftime_format(input: &str) -> IResult<&str, &str> {
    delimited('\'', take_while(1.., (AsChar::is_alpha, "%-")), '\'').parse_next(input)
}

pub fn parse_strftime_template(input: &str) -> IResult<&str, (&str, Vec<&str>, &str)> {
    let (_, template) = parse_template.parse_next(input)?;

    let (remainder, (function, (args, formatting_string))) = (
        parse_text_incl_underscore_hyphen,
        delimited(
            parse_function_boundary,
            separated_pair(
                parse_dot_separated_text,
                alt(((", "), (","))),
                parse_strftime_format,
            ),
            parse_function_boundary,
        ),
    )
        .parse_next(template)?;

    Ok((remainder, (function, args, formatting_string)))
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[should_panic]
    #[case("{{uppercase(metadata.extension)}}", ("", vec![""], ""))]
    #[should_panic]
    #[case("{{metadata.extension}}", ("", vec![""], ""))]
    #[case("{{strftime(metadata.date_added, '%Y-%m-%d')}}", ("strftime", vec!["metadata", "date_added"], "%Y-%m-%d"))]
    fn test_parse_strftime_template_passes(
        #[case] template: &str,
        #[case] outcome: (&str, Vec<&str>, &str),
    ) {
        let (remainder, result) = (parse_strftime_template).parse_next(template).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(result, outcome);
    }

    #[rstest]
    #[should_panic]
    #[case("{{strftime(metadata.date_added, '%Y-%m-%d')}}", ("", vec![""]))]
    #[should_panic]
    #[case("{{metadata.extension}}", ("", vec![""]))]
    #[case("{{uppercase(metadata.extension)}}", ("uppercase", vec!["metadata", "extension"]))]
    #[case("{{lowercase(metadata.name)}}", ("lowercase", vec!["metadata", "name"]))]
    fn test_parse_transform_template_passes(
        #[case] template: &str,
        #[case] outcome: (&str, Vec<&str>),
    ) {
        let (remainder, result) = (parse_transform_template).parse_next(template).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(result, outcome);
    }

    #[rstest]
    #[should_panic]
    #[case("{{uppercase(metadata.extension)}}", vec!["uppercase", "metadata", "extension"])]
    #[should_panic]
    #[case("{{strftime(metadata.date_added, '%Y-%m-%d')}}", vec![""])]
    #[case("{{utility.counter}}", vec!["utility", "counter"])]
    #[case("{{metadata.extension}}", vec!["metadata", "extension"])]
    #[case("{{metadata.created.year}}", vec!["metadata", "created", "year"])]
    #[case("{{content.customer}}", vec!["content", "customer"])]
    #[case("{{metadata.last_modified.year}}", vec!["metadata", "last_modified", "year"])]
    fn test_parse_dotted_template_passes(#[case] template: &str, #[case] outcome: Vec<&str>) {
        let (remainder, result) = (parse_dotted_template).parse_next(template).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(result, outcome);
    }
}
