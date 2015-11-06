#![feature(plugin)]
#![plugin(peg_syntax_ext)]

pub use self::param::*;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Equation{
    left: Operand,
    right: Operand
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Function{
    pub function:String,
    pub params:Vec<Operand>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Operand{
    Column(String),
    Function(Function),
    Value(String),
    Vec(Vec<Operand>),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Connector{
    AND,
    OR,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Direction{
    ASC,
    DESC,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Order{
    column: String,
    direction: Direction,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Equality{
    EQ, // = ,
    NEQ, // != ,
    LT, // <,
    LTE, // <=,
    GT, // >,
    GTE, // >=,
    IN, // IN
    NOT_IN,//NOT IN,
    IS,// IS
    IS_NOT,// IS NOT 
    LIKE, // LIKE
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Condition{
    pub left:Operand,
    pub equality:Equality,
    pub right:Operand,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Filter{
    connector: Option<Connector>,
    condition: Condition,
    subfilter: Vec<Filter>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Params{
    equations: Vec<Equation>,
    orders: Vec<Order>,
    filters: Vec<Filter>,
    conditions: Vec<Condition>,
}


peg! param(r#"
use super::*;


#[pub]
name -> String
  = [a-zA-Z0-9_]+ { match_str.to_string() }

#[pub]
equation -> Equation
    = l:operand "=" r:operand { Equation{left:l, right:r} }

#[pub]
operand -> Operand
	= c:name { Operand::Column(c) }

#[pub]
function -> Function
	= f:name "(" p:operand ")" { Function {function: f, params: vec![p]}}
	
#[pub]
equality -> Equality
	= "eq"     { Equality::EQ }
	/ "neq"    { Equality::NEQ }
	/ "lt" e:"e"?     { 
			match e { 
				None => Equality::LT,
				Some(e) => Equality::LTE, 
			} 
	}
	/ "gt" e:"e"?     { 
			match e { 
				None => Equality::GT,
				Some(e) => Equality::GTE, 
			} 
	}
    / "in"     { Equality::IN }
    / "not_in" { Equality::NOT_IN }
    / "is" _not:"_not"?     { 
			match _not { 
				None => Equality::IS,
				Some(e) => Equality::IS_NOT, 
			} 
	}
    / "like"   { Equality::LIKE }

#[pub]
condition -> Condition
	= l:operand "=" eq:equality "." r:operand {
		Condition{left: l, equality: eq, right: r}
	}
	/ "(" c:condition ")" { 
			c
	}
	

#[pub]
direction -> Direction
	= "asc" { Direction::ASC }
	/ "desc" { Direction::DESC }

#[pub]
order -> Order
	= c:name "." d:direction { Order{ column: c, direction: d} }

#[pub]
connector -> Connector
	= "&" { Connector::AND }
	/ "|" { Connector::OR }

#[pub]
filter -> Filter
	= lc:condition cc:connector_condition* {
		let mut sub_filters = vec![];
		for (conn, cond) in cc{
			let filter = Filter{ 
							connector: Some(conn), 
							condition: cond, 
							subfilter: vec![]
						};
			sub_filters.push(filter);
		}
		 
    	Filter {
    		connector: None,
    		condition:lc,
    		subfilter: sub_filters
    	}
	}
	/ c: condition{
		Filter{connector:None, condition:c, subfilter: vec![]}
	}
	
	/ "(" f:filter ")" { 
			f
	}
	/ lc:condition con:connector rf:filter {
        Filter {
        	connector: None,
        	condition: lc,
        	subfilter: vec![Filter{connector: Some(con), condition: rf.condition, subfilter: vec![]}]
        }
	}
	/ lf:filter conn_fil:connector_filter* {
		let mut sub_filters = vec![];
		for (conn, fil) in conn_fil{
			let filter = Filter{connector: Some(conn), condition: fil.condition, subfilter: vec![]};
			sub_filters.push(filter);
		}
        Filter {
        	connector: None,
        	condition: lf.condition,
        	subfilter: sub_filters
        }
	}

#[pub]
connector_condition -> (Connector, Condition)
	= con:connector rc:condition { (con, rc) }	

#[pub]
connector_filter -> (Connector, Filter)
	= con:connector rf:filter { (con, rf) }	
"#);

