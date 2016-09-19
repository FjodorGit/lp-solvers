use std::ops::{Add, Mul, Sub, Neg};
use variables::{LpExpression, LpConstraint, Constraint};
use variables::LpExpression::*;
use std::rc::Rc;

pub trait LpOperations<T> where T: Into<LpExpression> {
    fn le(&self, lhs_expr: T) -> LpConstraint;
    fn ge(&self, lhs_expr: T) -> LpConstraint;
    fn equal(&self, lhs_expr: T) -> LpConstraint;
}

impl Into<LpExpression> for i32 {
    fn into(self) -> LpExpression {
        LitVal(self)
    }
}

impl<'a> Into<LpExpression> for &'a LpExpression {
    fn into(self) -> LpExpression {
        self.clone()
    }
}


// <LpExr> op <LpExpr> where LpExpr is implicit
impl<T: Into<LpExpression> + Clone, U> LpOperations<T> for U where U: Into<LpExpression> + Clone {
    fn le(&self, lhs_expr: T) -> LpConstraint {
        LpConstraint(self.clone().into(), Constraint::LessOrEqual, lhs_expr.clone().into()).generalize()
    }
    fn ge(&self, lhs_expr: T) -> LpConstraint {
        LpConstraint(self.clone().into(), Constraint::GreaterOrEqual, lhs_expr.clone().into()).generalize()
    }
    fn equal( &self, lhs_expr: T) -> LpConstraint {
        LpConstraint(self.clone().into(), Constraint::Equal, lhs_expr.clone().into()).generalize()
    }
}


// LpExpr + (LpExpr, &LpExpr, i32)
impl<T> Add<T> for LpExpression where T: Into<LpExpression> + Clone {
    type Output = LpExpression;
    fn add(self, _rhs: T) -> LpExpression {
        AddExpr(Rc::new(self.clone()), Rc::new(_rhs.into()))
    }
}

// &LpExpr + (LpExpr, &LpExpr, i32)
impl<'a, T> Add<T> for &'a LpExpression where T: Into<LpExpression> + Clone {
    type Output = LpExpression;
    fn add(self, _rhs: T) -> LpExpression {
        AddExpr(Rc::new(self.clone()), Rc::new(_rhs.into()))
    }
}

// i32 + &LpExpr
impl<'a> Add<&'a LpExpression> for i32 {
    type Output = LpExpression;
    fn add(self, _rhs: &'a LpExpression) -> LpExpression {
        AddExpr(Rc::new(LitVal(self)), Rc::new(_rhs.clone()))
    }
}

// LpExpr - (LpExpr, &LpExpr, i32)
impl<T> Sub<T> for LpExpression where T: Into<LpExpression> + Clone {
    type Output = LpExpression;
    fn sub(self, _rhs: T) -> LpExpression {
        SubExpr(Rc::new(self.clone()), Rc::new(_rhs.into()))
    }
}

// &LpExpr - (LpExpr, &LpExpr, i32)
impl<'a, T> Sub<T> for &'a LpExpression where T: Into<LpExpression> + Clone {
    type Output = LpExpression;
    fn sub(self, _rhs: T) -> LpExpression {
        SubExpr(Rc::new(self.clone()), Rc::new(_rhs.into()))
    }
}

// i32 - &LpExpr
impl<'a> Sub<&'a LpExpression> for i32 {
    type Output = LpExpression;
    fn sub(self, _rhs: &'a LpExpression) -> LpExpression {
        SubExpr(Rc::new(LitVal(self)), Rc::new(_rhs.clone()))
    }
}

impl<'a> Neg for &'a LpExpression {
    type Output = LpExpression;
    fn neg(self) -> LpExpression {
        MulExpr(Rc::new(LitVal(-1)), Rc::new(self.clone()))
    }
}



// i32 * LpExpr
impl Mul<LpExpression> for i32 {
    type Output = LpExpression;
    fn mul(self, _rhs: LpExpression) -> LpExpression {
        LpExpression::MulExpr(Rc::new(LitVal(self)), Rc::new(_rhs))
    }
}

// i32 * &LpExp
impl<'a> Mul<&'a LpExpression> for i32 {
    type Output = LpExpression;

    fn mul(self, _rhs: &'a LpExpression) -> LpExpression {
        MulExpr(Rc::new(LitVal(self)), Rc::new(_rhs.clone()))
    }
}
