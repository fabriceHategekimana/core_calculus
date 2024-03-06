use crate::model::Data;
use crate::model::Group;
use crate::model::Instruction;
use nom::{
    IResult
};

pub fn parse(s: &str) -> (Vec<Data>, Vec<Instruction>) {
    todo!();
}


fn parse_symbol(s: &str) -> IResult<&str,Data> {
    todo!();
}

fn parse_group(s: &str) -> IResult<&str,Data> {
    todo!();
}


fn parse_eval(s: &str) -> IResult<&str,Data> {
    todo!();
}

fn parse_evaluation(s: &str) -> IResult<&str,Data> {
    todo!();
}

fn parse_atom(s: &str) -> IResult<&str,Data> {
    todo!();
}

fn parse_typing(s: &str) -> IResult<&str,Data> {
    todo!();
}

fn parse_calculus(s: &str) -> IResult<&str,Data> {
    todo!();
}

/* Calculus
 *  - Group
 *    - Symbol
 * Atom
 *  - Symbol
 *    - Evaluation
 *      - symbol
 *      - symbol
 *  - Typing
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_symbol(){
       assert_eq!(Data::Symbol("\"Hello\"".to_string()), parse_symbol("Hello").unwrap().1);
       assert_eq!(Data::Symbol("\"Hello world\"".to_string()), parse_symbol("Hello world").unwrap().1);
       assert_eq!(Data::Symbol("\"1234 old /lambda .\"".to_string()), parse_symbol("Hello world").unwrap().1);
    }

    #[test]
    fn test_parse_group() {
        assert_eq!(
                    parse_group("term t {\"hello\", \"world\"}").unwrap().1,
                    Data::Group(vec!["hello".to_string(), "world".to_string()])
                  );
    }

    #[test]
    fn test_parse_calculus() {
        assert_eq!(
            parse_calculus("calculus system_r { term t {\"hello\", \"world\"}}").unwrap().1,
            Data::Calculus(vec![Group(vec!["hello".to_string(), "world".to_string()])]));
    }
}

